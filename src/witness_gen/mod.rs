use std::{
    collections::{BTreeMap, btree_map},
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context, Result};
use fs_err as fs;
use handlebars::Handlebars;
use trustfall::FieldValue;
use trustfall_rustdoc::VersionedRustdocAdapter;

use crate::check_release::LintResult;
use crate::data_generation::{CrateDataRequest, effective_witness_rustflags};
use crate::query::{Witness, WitnessPurpose, WitnessQuery};
use crate::{GlobalConfig, SemverQuery, WitnessGeneration, WitnessStatistics};

mod artifacts;
mod script;
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;

use artifacts::{
    build_run_manifest, classify_execution_outcome, execute_script, final_artifact_relative_dir,
    final_run_root, maybe_write_script_lockfiles, retain_witness_error, temp_run_root,
    write_canonical_witness_script, write_executed_script_variants, write_execution_logs,
    write_query_result_file,
};
#[cfg(test)]
use script::{
    DependencyProfile, dependency_profile, determine_crate_root_name, render_script_variants,
};
use script::{GeneratedWitnessScript, build_witness_script};

const USER_REPRODUCTION_DRIVER: &str = "cargo +nightly -Zscript check";
const SCRIPT_SHEBANG: &str = "#!/usr/bin/env -S cargo +nightly -Zscript";
const RUN_RESULTS_DIR: &str = "run-results";
const TEMP_DIR: &str = "tmp";

/// Higher level data used specifically in the generation of a witness program. Values of [`None`]
/// for baseline/current mean witness execution is not possible for this source type.
pub(crate) struct WitnessGenerationData<'a> {
    baseline: Option<&'a CrateDataRequest<'a>>,
    current: Option<&'a CrateDataRequest<'a>>,
    target_dir: PathBuf,
}

impl<'a> WitnessGenerationData<'a> {
    pub(crate) fn new(
        baseline: Option<&'a CrateDataRequest<'a>>,
        current: Option<&'a CrateDataRequest<'a>>,
        target_dir: PathBuf,
    ) -> Self {
        Self {
            baseline,
            current,
            target_dir,
        }
    }

    fn requested_build_target(&self) -> Result<Option<&str>> {
        let baseline_target = self.baseline.and_then(CrateDataRequest::build_target);
        let current_target = self.current.and_then(CrateDataRequest::build_target);

        if baseline_target != current_target {
            anyhow::bail!(
                "witness generation expected baseline/current build targets to \
                 match, got baseline {baseline_target:?} and current \
                 {current_target:?}",
            );
        }

        Ok(baseline_target.or(current_target))
    }

    fn execution_config(&self) -> Result<WitnessExecutionConfig> {
        let build_target = self.requested_build_target()?.map(ToOwned::to_owned);
        let request_for_flags = self.current.or(self.baseline).context(
            "cannot determine witness execution flags: missing baseline and \
             current crate data request",
        )?;

        Ok(WitnessExecutionConfig {
            build_target,
            rustflags: effective_witness_rustflags(request_for_flags)?,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct WitnessManifestSummary {
    /// Count of witnesses that compiled on baseline and failed on current.
    pub(crate) confirmed_breaking: usize,
    /// Count of `RequiredForCorrectness` candidates that were suppressed because
    /// the witness also compiled on current.
    pub(crate) not_confirmed_by_witness: usize,
    /// Count of `ConsistencyCheck` query results whose witness still compiled on
    /// current.
    pub(crate) consistency_check_mismatches: usize,
    /// Count of witness execution failures while running consistency checks.
    pub(crate) consistency_check_errors: usize,
    /// Count of witness execution failures for required witnesses.
    pub(crate) required_witness_errors: usize,
}

impl WitnessManifestSummary {
    fn add(&mut self, other: &Self) {
        self.confirmed_breaking += other.confirmed_breaking;
        self.not_confirmed_by_witness += other.not_confirmed_by_witness;
        self.consistency_check_mismatches += other.consistency_check_mismatches;
        self.consistency_check_errors += other.consistency_check_errors;
        self.required_witness_errors += other.required_witness_errors;
    }
}

#[derive(Debug)]
pub(crate) struct WitnessRunReport {
    /// Crate name this witness report belongs to.
    pub(crate) crate_name: String,
    /// `target` directory root used for this crate's witness temp and retained
    /// artifacts.
    pub(crate) target_dir: PathBuf,
    /// User-facing witness statistics for this crate, if any were produced.
    pub(crate) statistics: Option<WitnessStatistics>,
    /// Aggregate counts that feed the retained run manifest.
    pub(crate) manifest_summary: WitnessManifestSummary,
    /// Provisional retained artifacts produced while evaluating witnesses for
    /// this crate. Finalization moves these from temp locations into their
    /// stable run-results layout.
    pub(crate) retained_artifacts: Vec<RetainedWitnessArtifact>,
}

impl WitnessRunReport {
    pub(crate) fn empty(crate_name: &str, target_dir: PathBuf) -> Self {
        Self {
            crate_name: crate_name.to_owned(),
            target_dir,
            statistics: None,
            manifest_summary: WitnessManifestSummary::default(),
            retained_artifacts: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RetainedArtifactStatus {
    NotConfirmedByWitness,
    ConsistencyCheckMismatch,
    ConsistencyCheckError,
    RequiredWitnessError,
}

impl RetainedArtifactStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::NotConfirmedByWitness => "not_confirmed_by_witness",
            Self::ConsistencyCheckMismatch => "consistency_check_mismatch",
            Self::ConsistencyCheckError => "consistency_check_error",
            Self::RequiredWitnessError => "required_witness_error",
        }
    }
}

#[derive(Debug)]
pub(crate) struct RetainedWitnessArtifact {
    /// Lint ID that produced this witness artifact.
    pub(crate) lint_id: String,
    /// Zero-based index of the lint query result within that lint.
    pub(crate) result_index: usize,
    /// Whether this witness was required for correctness or was only a
    /// consistency check.
    pub(crate) purpose: WitnessPurpose,
    /// Why this artifact was retained instead of being discarded on success.
    pub(crate) status: RetainedArtifactStatus,
    /// Provisional bundle directory under `tmp/run-<id>/...` before finalization.
    pub(crate) temp_artifact_dir: PathBuf,
    /// Exit code from checking the baseline script, if execution reached that
    /// point.
    pub(crate) baseline_exit_code: Option<i32>,
    /// Exit code from checking the current script, if execution reached that
    /// point.
    pub(crate) current_exit_code: Option<i32>,
    /// Effective build configuration that should be recorded in manifests and
    /// reproduction commands when present.
    execution_config: Option<WitnessExecutionConfig>,
}

#[derive(Debug)]
enum SingleWitnessOutcome {
    ConfirmedBreaking,
    NotConfirmedByWitness {
        retained_artifact: RetainedWitnessArtifact,
    },
    WitnessError {
        retained_artifact: Option<RetainedWitnessArtifact>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WitnessExecutionClassification {
    ConfirmedBreaking,
    NotConfirmedByWitness,
    WitnessError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WitnessExecutionConfig {
    build_target: Option<String>,
    rustflags: Option<String>,
}

struct WitnessEvaluation<'a> {
    crate_name: &'a str,
    semver_query: &'a SemverQuery,
    purpose: WitnessPurpose,
    result_index: usize,
    query_result: &'a BTreeMap<Arc<str>, FieldValue>,
}

struct GeneratedWitnessRun<'a> {
    temp_artifact_dir: &'a Path,
    witness_target_dir: &'a Path,
    run_id: &'a str,
    execution_config: WitnessExecutionConfig,
    generated_script: GeneratedWitnessScript,
}

/// Runs the witness query of a given [`WitnessQuery`] over a given lint query match, and merges
/// the witness query results with the existing lint results. Each query must match exactly once.
fn run_witness_query(
    adapter: &VersionedRustdocAdapter,
    witness_query: &WitnessQuery,
    mut lint_result: BTreeMap<Arc<str>, FieldValue>,
) -> Result<BTreeMap<Arc<str>, FieldValue>> {
    let arguments = witness_query
        .inherit_arguments_from(&lint_result)
        .context("error inheriting arguments in witness query")?;
    let arguments_debug = format!("{arguments:?}");

    let witness_results = adapter
        .run_query(&witness_query.query, arguments)
        .and_then(|mut query_results| {
            if let Some(query_result) = query_results.next() {
                match query_results.next() {
                    Some(extra_match) => Err(anyhow::anyhow!(
                        "witness query should match exactly one time, query \
                         matched producing both {query_result:?} and \
                         {extra_match:?}",
                    )),
                    None => Ok(query_result),
                }
            } else {
                Err(anyhow::anyhow!(
                    "witness query should match exactly one time, matched zero times"
                ))
            }
        })
        .with_context(|| {
            format!("error running witness query with input arguments {arguments_debug}")
        })?;

    for (key, value) in witness_results {
        match lint_result.entry(key) {
            btree_map::Entry::Vacant(entry) => {
                entry.insert(value);
            }
            btree_map::Entry::Occupied(entry) => anyhow::bail!(
                "witness query tried to output to existing key `{}`, \
                 overriding `{:?}` with `{value:?}`",
                entry.key(),
                entry.get(),
            ),
        }
    }

    Ok(lint_result)
}

fn generate_witness_text(
    handlebars: &Handlebars,
    witness_template: &str,
    witness_results: BTreeMap<Arc<str>, FieldValue>,
) -> Result<String> {
    let pretty_witness_data: BTreeMap<Arc<str>, trustfall::TransparentValue> = witness_results
        .into_iter()
        .map(|(k, v)| (k, v.into()))
        .collect();

    handlebars
        .render_template(witness_template, &pretty_witness_data)
        .context("error instantiating witness template")
}

/// Runs witnesses as a second pass over lint query results and rewrites each
/// lint's authoritative result set accordingly.
///
/// `RequiredForCorrectness` witnesses can suppress candidate results entirely.
/// `ConsistencyCheck` witnesses never change query results; they only add
/// statistics, warning summaries, and retained artifacts when they disagree
/// with or fail to check the query result.
pub(crate) fn run_witness_checks(
    config: &mut GlobalConfig,
    witness_generation: &WitnessGeneration,
    witness_data: WitnessGenerationData<'_>,
    crate_name: &str,
    adapter: &VersionedRustdocAdapter,
    lint_results: &mut [LintResult],
) -> WitnessRunReport {
    let mut report = WitnessRunReport::empty(crate_name, witness_data.target_dir.clone());
    let mut not_confirmed_by_witness = 0usize;
    let mut consistency_check_mismatches = 0usize;
    let mut consistency_check_errors = 0usize;
    let mut required_witness_errors = 0usize;

    for lint_result in lint_results {
        let Some(witness) = lint_result.semver_query.witness.as_ref() else {
            continue;
        };

        if !should_run_witness(witness_generation, witness) {
            continue;
        }

        let query_results = std::mem::take(&mut lint_result.query_results);
        let mut authoritative_results = Vec::with_capacity(query_results.len());

        for (result_index, query_result) in query_results.into_iter().enumerate() {
            let outcome = evaluate_single_witness(
                config,
                &witness_data,
                adapter,
                witness,
                WitnessEvaluation {
                    crate_name,
                    semver_query: &lint_result.semver_query,
                    purpose: witness.purpose,
                    result_index,
                    query_result: &query_result,
                },
            );

            match outcome {
                SingleWitnessOutcome::ConfirmedBreaking => {
                    report.manifest_summary.confirmed_breaking += 1;
                    authoritative_results.push(query_result);
                }
                SingleWitnessOutcome::NotConfirmedByWitness { retained_artifact } => {
                    report.manifest_summary.not_confirmed_by_witness += 1;
                    match witness.purpose {
                        WitnessPurpose::RequiredForCorrectness => {
                            not_confirmed_by_witness += 1;
                            config
                                .log_extra_verbose(|config| {
                                    config.shell_note(format_args!(
                                        "suppressed candidate result for `{}` because it was \
                                         not confirmed by witness",
                                        lint_result.semver_query.id
                                    ))
                                })
                                .expect("print failed");
                        }
                        WitnessPurpose::ConsistencyCheck => {
                            report.manifest_summary.consistency_check_mismatches += 1;
                            consistency_check_mismatches += 1;
                            authoritative_results.push(query_result);
                        }
                    }
                    report.retained_artifacts.push(retained_artifact);
                }
                SingleWitnessOutcome::WitnessError { retained_artifact } => {
                    match witness.purpose {
                        WitnessPurpose::RequiredForCorrectness => {
                            report.manifest_summary.required_witness_errors += 1;
                            required_witness_errors += 1;
                            config
                                .log_verbose(|config| {
                                    config.shell_error(format_args!(
                                        "required witness run failed for `{}`",
                                        lint_result.semver_query.id
                                    ))
                                })
                                .expect("print failed");
                        }
                        WitnessPurpose::ConsistencyCheck => {
                            report.manifest_summary.consistency_check_errors += 1;
                            consistency_check_errors += 1;
                            authoritative_results.push(query_result);
                        }
                    }
                    if let Some(retained_artifact) = retained_artifact {
                        report.retained_artifacts.push(retained_artifact);
                    }
                }
            }
        }

        lint_result.query_results = authoritative_results;
    }

    let statistics = WitnessStatistics::new(
        not_confirmed_by_witness,
        consistency_check_mismatches,
        consistency_check_errors,
        required_witness_errors,
    );
    report.statistics = (!statistics.is_empty()).then_some(statistics);
    report
}

/// Finalizes retained witness artifacts for one run ID.
///
/// Callers should invoke this after all crate reports have been collected. It
/// consumes the provisional bundles under `tmp/run-<id>/...`, removes that temp
/// tree when nothing was retained, moves retained artifacts into their stable
/// `run-results/run-<id>/...` layout, and writes the single manifest that
/// describes the finalized run.
pub(crate) fn finalize_retained_artifacts(
    run_id: &str,
    witness_run_reports: &[WitnessRunReport],
) -> Result<Vec<PathBuf>> {
    let mut reports_by_target_dir: BTreeMap<&Path, Vec<&WitnessRunReport>> = BTreeMap::new();
    for report in witness_run_reports {
        reports_by_target_dir
            .entry(report.target_dir.as_path())
            .or_default()
            .push(report);
    }

    let mut finalized_run_dirs = Vec::new();
    for (target_dir, reports) in reports_by_target_dir {
        let total_retained = reports
            .iter()
            .map(|report| report.retained_artifacts.len())
            .sum::<usize>();
        let temp_run_root = temp_run_root(target_dir, run_id);

        if total_retained == 0 {
            if temp_run_root.exists() {
                let _ = fs::remove_dir_all(&temp_run_root);
            }
            continue;
        }

        let final_run_root = final_run_root(target_dir, run_id);
        let final_artifact_root = final_run_root.join("artifacts");
        fs::create_dir_all(&final_artifact_root).with_context(|| {
            format!(
                "failed to create witness artifact root {}",
                final_artifact_root.display()
            )
        })?;

        for report in &reports {
            for artifact in &report.retained_artifacts {
                let relative_dir = final_artifact_relative_dir(
                    &report.crate_name,
                    &artifact.lint_id,
                    artifact.result_index,
                );
                let final_artifact_dir = final_run_root.join(&relative_dir);
                if let Some(parent) = final_artifact_dir.parent() {
                    fs::create_dir_all(parent).with_context(|| {
                        format!(
                            "failed to create witness artifact parent {}",
                            parent.display()
                        )
                    })?;
                }
                fs::rename(&artifact.temp_artifact_dir, &final_artifact_dir).with_context(
                    || {
                        format!(
                            "failed to move witness artifact from {} to {}",
                            artifact.temp_artifact_dir.display(),
                            final_artifact_dir.display()
                        )
                    },
                )?;

                let _ = maybe_write_script_lockfiles(&final_artifact_dir);
            }
        }

        let manifest = build_run_manifest(run_id, &final_run_root, &reports)?;

        let manifest_path = final_run_root.join("manifest.toml");
        fs::write(&manifest_path, toml::to_string_pretty(&manifest)?)
            .with_context(|| format!("failed to write {}", manifest_path.display()))?;

        if temp_run_root.exists() {
            let _ = fs::remove_dir_all(&temp_run_root);
        }

        finalized_run_dirs.push(final_run_root);
    }

    Ok(finalized_run_dirs)
}

fn should_run_witness(witness_generation: &WitnessGeneration, witness: &Witness) -> bool {
    match witness.purpose {
        WitnessPurpose::RequiredForCorrectness => true,
        WitnessPurpose::ConsistencyCheck => {
            witness_generation.run_consistency_checks && witness.witness_template.is_some()
        }
    }
}

fn evaluate_single_witness(
    config: &mut GlobalConfig,
    witness_data: &WitnessGenerationData<'_>,
    adapter: &VersionedRustdocAdapter,
    witness: &Witness,
    evaluation: WitnessEvaluation<'_>,
) -> SingleWitnessOutcome {
    let temp_artifact_dir = temp_run_root(&witness_data.target_dir, config.run_id())
        .join("artifacts")
        .join(crate::util::slugify(evaluation.crate_name))
        .join(crate::util::slugify(&evaluation.semver_query.id))
        .join(evaluation.result_index.to_string());
    if temp_artifact_dir.exists()
        && let Err(error) = fs::remove_dir_all(&temp_artifact_dir)
        && temp_artifact_dir.exists()
    {
        return witness_error_without_artifact(
            config,
            &evaluation,
            anyhow::anyhow!(error).context(format!(
                "failed to remove pre-existing witness temp dir {}",
                temp_artifact_dir.display()
            )),
        );
    }
    if let Err(error) = fs::create_dir_all(&temp_artifact_dir) {
        return witness_error_without_artifact(
            config,
            &evaluation,
            anyhow::anyhow!(error).context(format!(
                "failed to create witness temp dir {}",
                temp_artifact_dir.display()
            )),
        );
    }

    let generated_script = match build_witness_script(
        config.handlebars(),
        witness_data,
        adapter,
        witness,
        evaluation.query_result,
    ) {
        Ok(script) => script,
        Err(error) => {
            return retain_witness_error(
                &evaluation,
                &temp_artifact_dir,
                None,
                None,
                None,
                None,
                Some(anyhow::anyhow!(error)),
            );
        }
    };

    let execution_config = match witness_data.execution_config() {
        Ok(config) => config,
        Err(error) => {
            return retain_witness_error(
                &evaluation,
                &temp_artifact_dir,
                Some(&generated_script),
                None,
                None,
                None,
                Some(error),
            );
        }
    };

    let run_id = config.run_id().to_owned();
    evaluate_generated_witness(
        config,
        GeneratedWitnessRun {
            temp_artifact_dir: &temp_artifact_dir,
            witness_target_dir: &witness_data.target_dir,
            run_id: &run_id,
            execution_config,
            generated_script,
        },
        evaluation,
    )
}

/// Evaluates one fully-rendered witness script inside its provisional artifact
/// directory.
///
/// Callers should pass the same temp directory that may later be retained for
/// debugging. The fast path writes only the baseline/current scripts needed for
/// `cargo check`; retained-only metadata such as `witness.rs`, `query-result.toml`,
/// and execution logs is written only after an outcome requires keeping the bundle.
fn evaluate_generated_witness(
    config: &mut GlobalConfig,
    run: GeneratedWitnessRun<'_>,
    evaluation: WitnessEvaluation<'_>,
) -> SingleWitnessOutcome {
    if let Err(error) = write_executed_script_variants(run.temp_artifact_dir, &run.generated_script)
    {
        return retain_witness_error(
            &evaluation,
            run.temp_artifact_dir,
            Some(&run.generated_script),
            None,
            None,
            Some(&run.execution_config),
            Some(error),
        );
    }

    let baseline_execution = execute_script(
        temp_run_root(run.witness_target_dir, run.run_id),
        run.temp_artifact_dir,
        &run.execution_config,
        "baseline",
        "baseline.rs",
    );
    let current_execution = execute_script(
        temp_run_root(run.witness_target_dir, run.run_id),
        run.temp_artifact_dir,
        &run.execution_config,
        "current",
        "current.rs",
    );

    match (baseline_execution, current_execution) {
        (Ok(baseline), Ok(current)) => {
            config
                .log_extra_verbose(|config| {
                    config.shell_note(format_args!(
                        "witness `{}` result #{}: baseline={}, current={}",
                        evaluation.semver_query.id,
                        evaluation.result_index,
                        baseline.success,
                        current.success
                    ))
                })
                .expect("print failed");

            let classification = classify_execution_outcome(baseline.success, current.success);
            match classification {
                WitnessExecutionClassification::ConfirmedBreaking => {
                    if run.temp_artifact_dir.exists() {
                        let _ = fs::remove_dir_all(run.temp_artifact_dir);
                    }
                    SingleWitnessOutcome::ConfirmedBreaking
                }
                WitnessExecutionClassification::NotConfirmedByWitness
                | WitnessExecutionClassification::WitnessError => {
                    if let Err(error) =
                        write_canonical_witness_script(run.temp_artifact_dir, &run.generated_script)
                    {
                        return retain_witness_error(
                            &evaluation,
                            run.temp_artifact_dir,
                            Some(&run.generated_script),
                            Some(&baseline),
                            Some(&current),
                            Some(&run.execution_config),
                            Some(error),
                        );
                    }
                    if let Err(error) = write_query_result_file(
                        run.temp_artifact_dir,
                        evaluation.crate_name,
                        evaluation.semver_query,
                        evaluation.purpose,
                        evaluation.result_index,
                        &run.generated_script,
                        evaluation.query_result,
                    ) {
                        return retain_witness_error(
                            &evaluation,
                            run.temp_artifact_dir,
                            Some(&run.generated_script),
                            Some(&baseline),
                            Some(&current),
                            Some(&run.execution_config),
                            Some(error),
                        );
                    }
                    if let Err(error) =
                        write_execution_logs(run.temp_artifact_dir, &baseline, &current)
                    {
                        return retain_witness_error(
                            &evaluation,
                            run.temp_artifact_dir,
                            Some(&run.generated_script),
                            Some(&baseline),
                            Some(&current),
                            Some(&run.execution_config),
                            Some(error),
                        );
                    }

                    let status = match (classification, evaluation.purpose) {
                        (
                            WitnessExecutionClassification::NotConfirmedByWitness,
                            WitnessPurpose::RequiredForCorrectness,
                        ) => RetainedArtifactStatus::NotConfirmedByWitness,
                        (
                            WitnessExecutionClassification::NotConfirmedByWitness,
                            WitnessPurpose::ConsistencyCheck,
                        ) => RetainedArtifactStatus::ConsistencyCheckMismatch,
                        (
                            WitnessExecutionClassification::WitnessError,
                            WitnessPurpose::RequiredForCorrectness,
                        ) => RetainedArtifactStatus::RequiredWitnessError,
                        (
                            WitnessExecutionClassification::WitnessError,
                            WitnessPurpose::ConsistencyCheck,
                        ) => RetainedArtifactStatus::ConsistencyCheckError,
                        (WitnessExecutionClassification::ConfirmedBreaking, _) => {
                            unreachable!("confirmed witnesses are not retained")
                        }
                    };

                    let retained_artifact = RetainedWitnessArtifact {
                        lint_id: evaluation.semver_query.id.clone(),
                        result_index: evaluation.result_index,
                        purpose: evaluation.purpose,
                        status,
                        temp_artifact_dir: run.temp_artifact_dir.to_path_buf(),
                        baseline_exit_code: baseline.exit_code,
                        current_exit_code: current.exit_code,
                        execution_config: Some(run.execution_config.clone()),
                    };

                    match classification {
                        WitnessExecutionClassification::NotConfirmedByWitness => {
                            SingleWitnessOutcome::NotConfirmedByWitness { retained_artifact }
                        }
                        WitnessExecutionClassification::WitnessError => {
                            SingleWitnessOutcome::WitnessError {
                                retained_artifact: Some(retained_artifact),
                            }
                        }
                        WitnessExecutionClassification::ConfirmedBreaking => {
                            unreachable!("confirmed witnesses are not retained")
                        }
                    }
                }
            }
        }
        (baseline, current) => {
            let mut execution_errors = Vec::new();
            let baseline_execution = match baseline {
                Ok(execution) => Some(execution),
                Err(error) => {
                    execution_errors.push(format!("baseline: {error:#}"));
                    None
                }
            };
            let current_execution = match current {
                Ok(execution) => Some(execution),
                Err(error) => {
                    execution_errors.push(format!("current: {error:#}"));
                    None
                }
            };
            let error = if execution_errors.is_empty() {
                anyhow::anyhow!("failed to execute witness scripts")
            } else {
                anyhow::anyhow!(
                    "failed to execute witness scripts\n\n{}",
                    execution_errors.join("\n\n")
                )
            };

            retain_witness_error(
                &evaluation,
                run.temp_artifact_dir,
                Some(&run.generated_script),
                baseline_execution.as_ref(),
                current_execution.as_ref(),
                Some(&run.execution_config),
                Some(error),
            )
        }
    }
}

fn witness_error_without_artifact(
    config: &mut GlobalConfig,
    evaluation: &WitnessEvaluation<'_>,
    error: anyhow::Error,
) -> SingleWitnessOutcome {
    config
        .log_verbose(|config| {
            config.shell_error(format_args!(
                "witness `{}` result #{} failed before artifact capture: \
                 {error:#}",
                evaluation.semver_query.id, evaluation.result_index
            ))
        })
        .expect("print failed");

    SingleWitnessOutcome::WitnessError {
        retained_artifact: None,
    }
}
