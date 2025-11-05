use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::io::Write as _;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anstyle::{AnsiColor, Color, Reset, Style};

use anyhow::Context;
use clap::crate_version;
use itertools::{Either, Itertools};
use rayon::prelude::*;
use trustfall::{FieldValue, TransparentValue};

use crate::data_generation::DataStorage;
use crate::query::{
    ActualSemverUpdate, LintLevel, LintLogic, OverrideStack, RequiredSemverUpdate, SemverQuery,
};
use crate::witness_gen::{
    self,
    results::{WitnessCheckResult, WitnessChecksResultKind, WitnessLogicKinds},
};
use crate::{Bumps, CrateReport, GlobalConfig, ReleaseType, WitnessGeneration};

/// Represents a change between two semantic versions
#[derive(Debug, PartialEq, Eq)]
struct VersionChange {
    /// The level of the semantic version update (major, minor, patch)
    level: ActualSemverUpdate,
    /// Whether this is an actual change between different versions
    /// or a minimum possible change for the same version
    kind: VersionChangeKind,
}

#[derive(Debug, PartialEq, Eq)]
enum VersionChangeKind {
    Actual,
    Minimum,
}

/// Classifies the minimum semantic version change between two versions.
/// It would panic if either version is not a valid semver.
fn classify_minimum_semver_version_change(
    current_version: &str,
    baseline_version: &str,
) -> VersionChange {
    let baseline_version =
        semver::Version::parse(baseline_version).expect("baseline not a valid version");
    let current_version =
        semver::Version::parse(current_version).expect("current not a valid version");

    // Check if versions are identical (ignoring build metadata)
    if baseline_version.cmp_precedence(&current_version) == Ordering::Equal {
        if !baseline_version.pre.is_empty() {
            // the baseline is a pre-release: the minimum "next" change is major
            return VersionChange {
                level: ActualSemverUpdate::Major,
                kind: VersionChangeKind::Minimum,
            };
        }
        return get_minimum_version_change(&current_version);
    }

    // From the cargo reference:
    // > Initial development releases starting with "0.y.z" can treat changes
    // > in "y" as a major release, and "z" as a minor release.
    // > "0.0.z" releases are always major changes. This is because Cargo uses
    // > the convention that only changes in the left-most non-zero component
    // > are considered incompatible.
    // https://doc.rust-lang.org/cargo/reference/semver.html
    let update_kind = if baseline_version.major != current_version.major {
        ActualSemverUpdate::Major
    } else if baseline_version.minor != current_version.minor {
        if current_version.major == 0 {
            ActualSemverUpdate::Major
        } else {
            ActualSemverUpdate::Minor
        }
    } else if baseline_version.patch != current_version.patch {
        if current_version.major == 0 {
            if current_version.minor == 0 {
                ActualSemverUpdate::Major
            } else {
                ActualSemverUpdate::Minor
            }
        } else {
            ActualSemverUpdate::Patch
        }
    } else if baseline_version.pre != current_version.pre {
        // > A pre-release version indicates that the version is unstable and might not satisfy
        // > the intended compatibility requirements as denoted by its associated normal version
        // https://semver.org/#spec-item-9
        ActualSemverUpdate::Major
    } else {
        unreachable!(
            "versions have identical major.minor.patch components, but did not \
                register as equal when compared: \
                {current_version:?} vs {baseline_version:?}"
        );
    };

    VersionChange {
        level: update_kind,
        kind: VersionChangeKind::Actual,
    }
}

fn get_minimum_version_change(version: &semver::Version) -> VersionChange {
    let update = match (version.major, version.minor) {
        // For 0.0.z: Minimum next change must be major
        (0, 0) => ActualSemverUpdate::Major,
        // For 0.y.z: Minimum next change must be minor
        (0, _) => ActualSemverUpdate::Minor,
        // For x.y.z: Minimum next change must be patch
        (_, _) => ActualSemverUpdate::Patch,
    };

    VersionChange {
        level: update,
        kind: VersionChangeKind::Minimum,
    }
}

/// Intermediate state in `run_check_release`
#[derive(Debug)]
pub(crate) struct LintResult {
    pub semver_query: SemverQuery,
    pub query_results: Vec<BTreeMap<Arc<str>, FieldValue>>,
    /// Any results from witnesses for this lint
    pub witness_results: Option<WitnessCheckResult>,
    /// How long it took to run the semver query
    pub query_duration: Duration,
    /// Applied `OverrideStack`
    pub effective_required_update: RequiredSemverUpdate,
    pub effective_lint_level: LintLevel,
}

impl LintResult {
    pub fn should_pass(&self) -> bool {
        match &self.semver_query.lint_logic {
            LintLogic::UseStandard => self.query_results.is_empty(),
            // If we are using witness lint logic, this should pass either if the
            // witnesses never found any breaking changes, or if no witness results
            // are present
            LintLogic::UseWitness(_) => self
                .witness_results
                .as_ref()
                .is_none_or(|witness| !witness.breaking_change_found),
        }
    }
}

/// Helper function to print details about a triggered lint.
fn print_triggered_lint(
    config: &mut GlobalConfig,
    lint_result: &LintResult,
    witness_generation: &WitnessGeneration,
) -> anyhow::Result<()> {
    let semver_query = &lint_result.semver_query;
    if let Some(ref_link) = semver_query.reference_link.as_deref() {
        config.log_info(|config| {
            writeln!(config.stdout(), "{}Description:{}\n{}\n{:>12} {}\n{:>12} https://github.com/obi1kenobi/cargo-semver-checks/tree/v{}/src/lints/{}.ron\n",
                Style::new().bold(), Reset,
                &semver_query.error_message,
                "ref:",
                ref_link,
                "impl:",
                crate_version!(),
                semver_query.id,
            )?;
            Ok(())
        })?;
    } else {
        config.log_info(|config| {
            writeln!(
                config.stdout(),
                "{}Description:{}\n{}\n{:>12} https://github.com/obi1kenobi/cargo-semver-checks/tree/v{}/src/lints/{}.ron",
                Style::new().bold(),
                Reset,
                &semver_query.error_message,
                "impl:",
                crate_version!(),
                semver_query.id,
            )?;
            Ok(())
        })?;
    }

    config.log_info(|config| {
        writeln!(
            config.stdout(),
            "{}Failed in:{}",
            Style::new().bold(),
            Reset
        )?;
        Ok(())
    })?;

    // Boxed dynamic dispatch is only used for witness-based lints which override the lint results,
    // and is used since each Vec/Iterator can have its own type signature of other correlated information,
    // and as such will end up having its own mapping function.
    let results: Either<_, Box<dyn Iterator<Item = _>>> = match &lint_result.witness_results {
        None => Either::Left(lint_result.query_results.iter()),
        Some(WitnessCheckResult { check_results, .. }) => match check_results {
            WitnessChecksResultKind::Standard(_) => Either::Left(lint_result.query_results.iter()),
            WitnessChecksResultKind::WitnessLogic(WitnessLogicKinds::ExtractFuncArgs(results)) => {
                Either::Right(Box::new(
                    // These errors are addressed elwhere too, they can be ignored here
                    results.iter().filter_map(|result| {
                        result
                            .as_ref()
                            .ok()
                            .and_then(|(status, values)| status.is_breaking().then_some(values))
                    }),
                ))
            }
        },
    };

    for semver_violation_result in results {
        let pretty_result = semver_violation_result
            .iter()
            .map(|(k, v)| (&**k, v.clone().into()))
            .collect::<BTreeMap<&str, TransparentValue>>();

        if let Some(template) = semver_query.per_result_error_template.as_deref() {
            let message = config
                .handlebars()
                .render_template(template, &pretty_result)
                .context("Error instantiating semver query template.")
                .expect("could not materialize template");
            config.log_info(|config| {
                writeln!(config.stdout(), "  {message}")?;
                Ok(())
            })?;

            config.log_extra_verbose(|config| {
                let serde_pretty =
                    serde_json::to_string_pretty(&pretty_result).expect("serde failed");
                let indented_serde = serde_pretty
                    .split('\n')
                    .map(|line| format!("    {line}"))
                    .join("\n");
                writeln!(
                    config.stdout(),
                    "\tlint rule output values:\n{indented_serde}"
                )?;
                Ok(())
            })?;
        } else {
            config.log_info(|config| {
                writeln!(
                    config.stdout(),
                    "{}\n",
                    serde_json::to_string_pretty(&pretty_result)?
                )?;
                Ok(())
            })?;
        }

        if let Some(witness) = &semver_query.witness
            && witness_generation.show_hints
        {
            let message = config
                .handlebars()
                .render_template(&witness.hint_template, &pretty_result)
                .context("Error instantiating witness hint template.")?;

            config.log_info(|config| {
                let note = Style::new()
                    .fg_color(Some(Color::Ansi(AnsiColor::Cyan)))
                    .bold();
                writeln!(
                    config.stdout(),
                    "{note}note:{note:#} downstream code similar to the following would break:\n\
                        {message}\n"
                )?;
                Ok(())
            })?;
        }
    }

    Ok(())
}

fn print_errored_or_failed_witness(
    config: &mut GlobalConfig,
    check_result: &WitnessCheckResult,
) -> anyhow::Result<()> {
    let failures = check_result.get_errors_and_failures();

    // Only [`LintLogic::UseStandard`] logic cares about failures. They are repurposed in [`LintLogic::UseWitness`]
    // to inform lint success.
    if check_result.is_standard_logic() && !failures.failed_status.is_empty() {
        config.log_info(|config| {
            writeln!(config.stdout(), "{}Failed:{}", Style::new().bold(), Reset)?;
            Ok(())
        })?;

        for check_info in &failures.failed_status {
            config.log_info(|config| {
                        writeln!(
                            config.stdout(),
                            "  validation witness {}-{} failed, no breaking change occured when cargo-semver-checks detected one",
                            check_info.witness_name,
                            check_info.index
                        )?;
                        Ok(())
                    })?;
        }

        config.log_info(|config| {
            writeln!(config.stdout())?;
            Ok(())
        })?;
    }

    if !failures.errored_status.is_empty() || !failures.errors.is_empty() {
        config.log_info(|config| {
            writeln!(config.stdout(), "{}Errored:{}", Style::new().bold(), Reset,)?;
            Ok(())
        })?;

        for check_info in failures.errored_status {
            config.log_info(|config| {
                    writeln!(
                        config.stdout(),
                        "  witness {}-{} failed as an error, where baseline check {}, and current check {}",
                        check_info.info.witness_name,
                        check_info.info.index,
                        format_exit_status(&check_info.baseline_status),
                        format_exit_status(&check_info.current_status),
                    )?;
                    Ok(())
                })?;
        }

        for err in failures.errors {
            config.log_info(|config| {
                writeln!(
                    config.stdout(),
                    "  witness failed as an error with the error message: {err} (root cause: {})",
                    err.root_cause()
                )?;
                Ok(())
            })?;
        }

        config.log_info(|config| {
            writeln!(config.stdout())?;
            Ok(())
        })?;
    }

    Ok(())
}

fn format_exit_status(status: &std::process::ExitStatus) -> String {
    if status.success() {
        "exited successfully".to_string()
    } else if let Some(code) = status.code() {
        format!("exited with error code {code}")
    } else {
        "exited with an unknown error".to_string()
    }
}

#[expect(clippy::too_many_arguments)]
pub(super) fn run_check_release(
    config: &mut GlobalConfig,
    data_storage: &DataStorage,
    crate_name: &str,
    release_type: Option<ReleaseType>,
    overrides: &OverrideStack,
    witness_generation: &WitnessGeneration,
    witness_data: witness_gen::WitnessGenerationData,
    witness_rustdoc_paths: witness_gen::WitnessRustdocPaths,
) -> anyhow::Result<CrateReport> {
    let current_version = data_storage.current_crate().crate_version();
    let baseline_version = data_storage.baseline_crate().crate_version();

    let version_change = match release_type {
        // Case 1: User explicitly specified a release type
        Some(rt) => VersionChange {
            level: rt.into(),
            kind: VersionChangeKind::Actual,
        },
        // Case 2: Try to determine from version strings
        None => match (baseline_version, current_version) {
            (Some(baseline), Some(current)) => {
                classify_minimum_semver_version_change(baseline, current)
            }
            // Case 3: Fall back to assuming no change
            _ => {
                config
                    .shell_warn(
                        "Could not determine whether crate version changed. Assuming no change.",
                    )
                    .expect("print failed");
                VersionChange {
                    level: ActualSemverUpdate::NotChanged,
                    kind: VersionChangeKind::Actual,
                }
            }
        },
    };
    let change = match version_change.level {
        ActualSemverUpdate::Major => "major",
        ActualSemverUpdate::Minor => "minor",
        ActualSemverUpdate::Patch => "patch",
        ActualSemverUpdate::NotChanged => "no",
    };
    let assume = if release_type.is_some() || version_change.kind == VersionChangeKind::Minimum {
        "assume "
    } else {
        ""
    };

    let change_message = match version_change.kind {
        VersionChangeKind::Actual => format!("{assume}{change} change"),
        VersionChangeKind::Minimum => format!("no change; {assume}{change}"),
    };

    let index_storage = data_storage.create_indexes();
    let adapter = index_storage.create_adapter();

    let mut queries_to_run = SemverQuery::all_queries();
    let all_queries_len = queries_to_run.len();
    queries_to_run.retain(|_, query| {
        !version_change
            .level
            .supports_requirement(overrides.effective_required_update(query))
            && overrides.effective_lint_level(query) > LintLevel::Allow
    });
    let selected_checks = queries_to_run.len();
    let skipped_checks = all_queries_len - selected_checks;

    config.shell_status(
        "Checking",
        format_args!(
            "{crate_name} v{} -> v{} ({})",
            baseline_version.unwrap_or("unknown"),
            current_version.unwrap_or("unknown"),
            change_message
        ),
    )?;
    config
        .log_verbose(|config| {
            let current_num_threads = rayon::current_num_threads();
            if current_num_threads == 1 {
                config.shell_status(
                    "Starting",
                    format_args!("{} checks, {} unnecessary", selected_checks, skipped_checks),
                )
            } else {
                config.shell_status(
                    "Starting",
                    format_args!(
                        "{} checks, {} unnecessary on {current_num_threads} threads",
                        selected_checks, skipped_checks
                    ),
                )
            }
        })
        .expect("print failed");

    let checks_start_instant = Instant::now();
    let mut lint_results = queries_to_run
        .into_par_iter()
        .map(|(_, semver_query)| {
            let start_instant = std::time::Instant::now();
            // trustfall::execute_query(...) -> dyn Iterator (without Send)
            // thus the result must be collect()'ed
            let query_results = adapter
                .run_query(&semver_query.query, semver_query.arguments.clone())?
                .collect_vec();
            let query_duration = start_instant.elapsed();
            Ok(LintResult {
                effective_required_update: overrides.effective_required_update(&semver_query),
                effective_lint_level: overrides.effective_lint_level(&semver_query),
                witness_results: None,
                semver_query,
                query_duration,
                query_results,
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    let checks_duration = checks_start_instant.elapsed();

    let witness_report = if witness_generation.generate_witnesses {
        let witness_results = witness_gen::run_witness_checks(
            config,
            witness_data,
            witness_rustdoc_paths,
            crate_name,
            &adapter,
            &mut lint_results,
        );

        if let Err(error) = &witness_results {
            config.log_info(|config| {
                config.shell_warn(format!(
                    "encountered non-fatal error while generating witnesses for crate {crate_name}: {error} (root cause: {})",
                    error.root_cause()
                ))
            })?;
        }

        witness_results.ok()
    } else {
        None
    };

    let mut required_bumps = Bumps { major: 0, minor: 0 };
    let mut suggested_bumps = Bumps { major: 0, minor: 0 };
    for result in &lint_results {
        if !result.query_results.is_empty() {
            let bump_stats = match result.effective_lint_level {
                LintLevel::Deny => &mut required_bumps,
                LintLevel::Warn => &mut suggested_bumps,
                LintLevel::Allow => unreachable!(
                    "`LintLevel::Allow` lint was unexpectedly not skipped: {:?}",
                    result.semver_query
                ),
            };
            if result.semver_query.lint_logic.is_standard() {
                match result.effective_required_update {
                    RequiredSemverUpdate::Major => bump_stats.major += 1,
                    RequiredSemverUpdate::Minor => bump_stats.minor += 1,
                };
                // If we're using witness logic, only mark for bump if the witnesses ran, and if a breaking change was found
            } else if let Some(witness_results) = &result.witness_results
                && witness_results.breaking_change_found
            {
                match result.effective_required_update {
                    RequiredSemverUpdate::Major => bump_stats.major += 1,
                    RequiredSemverUpdate::Minor => bump_stats.minor += 1,
                };
            }
        }
    }

    let report = CrateReport {
        lint_results,
        checks_duration,
        selected_checks,
        skipped_checks,
        required_bumps,
        suggested_bumps,
        detected_bump: version_change.level,
        witness_report,
    };

    print_report(config, witness_generation, &report)?;
    Ok(report)
}

fn print_report(
    config: &mut GlobalConfig,
    witness_generation: &WitnessGeneration,
    report: &CrateReport,
) -> anyhow::Result<()> {
    let mut results_with_errors = vec![];
    let mut results_with_warnings = vec![];

    for result in &report.lint_results {
        config
            .log_verbose(|config| {
                let category = match result.effective_required_update {
                    RequiredSemverUpdate::Major => "major",
                    RequiredSemverUpdate::Minor => "minor",
                };

                let (status, status_color) =
                    match (result.should_pass(), result.effective_lint_level) {
                        (true, _) => ("PASS", AnsiColor::Green),
                        (false, LintLevel::Deny) => ("FAIL", AnsiColor::Red),
                        (false, LintLevel::Warn) => ("WARN", AnsiColor::Yellow),
                        (false, LintLevel::Allow) => unreachable!(
                            "`LintLevel::Allow` lint was unexpectedly not skipped: {:?}",
                            result.semver_query
                        ),
                    };

                writeln!(
                    config.stderr(),
                    "{}{:>12}{} [{:8.3}s] {:^18} {}",
                    Style::new()
                        .fg_color(Some(Color::Ansi(status_color)))
                        .bold(),
                    status,
                    Reset,
                    result.query_duration.as_secs_f32(),
                    category,
                    result.semver_query.id
                )?;
                Ok(())
            })
            .expect("print failed");

        if !result.query_results.is_empty() && !result.should_pass() {
            match result.effective_lint_level {
                LintLevel::Deny => results_with_errors.push(result),
                LintLevel::Warn => results_with_warnings.push(result),
                LintLevel::Allow => unreachable!(
                    "`LintLevel::Allow` lint was unexpectedly not skipped: {:?}",
                    result.semver_query
                ),
            };
        }
    }

    let produced_errors = !results_with_errors.is_empty();
    let produced_warnings = !results_with_warnings.is_empty();
    if produced_errors || produced_warnings {
        let status_color = if produced_errors {
            AnsiColor::Red
        } else {
            AnsiColor::Yellow
        };
        config
            .shell_print(
                "Checked",
                format_args!(
                    "[{:>8.3}s] {} checks: {} pass, {} fail, {} warn, {} skip",
                    report.checks_duration.as_secs_f32(),
                    report.selected_checks,
                    report.selected_checks
                        - results_with_errors.len()
                        - results_with_warnings.len(),
                    results_with_errors.len(),
                    results_with_warnings.len(),
                    report.skipped_checks,
                ),
                Color::Ansi(status_color),
                true,
            )
            .expect("print failed");

        for lint_result in results_with_errors {
            config.log_info(|config| {
                writeln!(
                    config.stdout(),
                    "\n--- failure {}: {} ---\n",
                    lint_result.semver_query.id,
                    lint_result.semver_query.human_readable_name
                )?;
                Ok(())
            })?;

            print_triggered_lint(config, lint_result, witness_generation)?;
        }

        for lint_result in results_with_warnings {
            config.log_info(|config| {
                writeln!(
                    config.stdout(),
                    "\n--- warning {}: {} ---\n",
                    lint_result.semver_query.id,
                    lint_result.semver_query.human_readable_name
                )?;
                Ok(())
            })?;

            print_triggered_lint(config, lint_result, witness_generation)?;
        }

        if let Some(required_bump) = report.required_bumps.update_type() {
            writeln!(config.stderr())?;
            config.shell_print(
                "Summary",
                format_args!(
                    "semver requires new {} version: {} major and {} minor checks failed",
                    required_bump.as_str(),
                    report.required_bumps.major,
                    report.required_bumps.minor,
                ),
                Color::Ansi(AnsiColor::Red),
                true,
            )?;
        } else if produced_warnings {
            writeln!(config.stderr())?;
            config.shell_print(
                "Summary",
                "no semver update required",
                Color::Ansi(AnsiColor::Green),
                true,
            )?;
        } else {
            unreachable!("Expected either warnings or errors to be produced.");
        }

        if let Some(suggested_bump) = report.suggested_bumps.update_type() {
            config.shell_print(
                "Warning",
                format_args!(
                    "produced {} major and {} minor level warnings",
                    report.suggested_bumps.major, report.suggested_bumps.minor,
                ),
                Color::Ansi(AnsiColor::Yellow),
                true,
            )?;

            if report
                .required_bumps
                .update_type()
                .is_none_or(|required_bump| required_bump < suggested_bump)
            {
                writeln!(
                    config.stderr(),
                    "{:12} produced warnings suggest new {} version",
                    "",
                    suggested_bump.as_str(),
                )?;
            }
        }
    } else {
        config
            .shell_print(
                "Checked",
                format_args!(
                    "[{:>8.3}s] {} checks: {} pass, {} skip",
                    report.checks_duration.as_secs_f32(),
                    report.selected_checks,
                    report.selected_checks,
                    report.skipped_checks,
                ),
                Color::Ansi(AnsiColor::Green),
                true,
            )
            .expect("print failed");

        config.shell_print(
            "Summary",
            "no semver update required",
            Color::Ansi(AnsiColor::Green),
            true,
        )?;
    }

    if let Some(witness_report) = &report.witness_report {
        writeln!(config.stderr())?;
        let status_color = if witness_report.failed() {
            AnsiColor::Red
        } else {
            AnsiColor::Green
        };
        config
            .shell_print(
                "Witnesses",
                format_args!(
                    "[{:>8.3}s] {} witnesses for {} lints: {} succeeded, {} repurposed failures, {} failed, {} errored",
                    witness_report.time_elapsed.as_secs_f32(),
                    witness_report.generated_checks,
                    witness_report.lints_with_checks,
                    witness_report.succeeded_checks,
                    witness_report.repurposed_failed_checks,
                    witness_report.failed_checks,
                    witness_report.errored_checks
                ),
                Color::Ansi(status_color),
                true,
            )?;
        config.log_info(|config| {
            // TODO: Add more information here about witnesses
            writeln!(
                config.stdout(),
                "\n{}Information:{}\nWitnesses of failures have been generated, if applicable.\n{:>12} {}\n",
                Style::new().bold(),
                Reset,
                "path:",
                witness_report.witness_set_dir.display()
            )?;
            Ok(())
        })?;

        if let Err(err) = &witness_report.deletion_error {
            config.log_info(|config| {
                writeln!(
                    config.stdout(),
                    "\n{}Warning:{}\nA witness failed to delete successfully. Any other witnesses that \
                    would have been deleted were not deleted.\n{:>12} {err} (root cause: {})\n",
                    Style::new().bold(),
                    Reset,
                    "error:",
                    err.root_cause()
                )?;
                Ok(())
            })?;
        }

        for (lint_result, check_result) in report.lint_results.iter().filter_map(|result| {
            result
                .witness_results
                .as_ref()
                .and_then(|check_result| check_result.has_errored_check.then_some(check_result))
                .map(|check_result| (result, check_result))
        }) {
            config.log_info(|config| {
                writeln!(
                    config.stdout(),
                    "\n--- witness failure {}: {} ---\n",
                    lint_result.semver_query.id,
                    lint_result.semver_query.human_readable_name
                )?;
                Ok(())
            })?;

            print_errored_or_failed_witness(config, check_result)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn classify_same_version() {
        let baseline = "1.0.0";
        let current = "1.0.0";
        let expected = VersionChange {
            level: ActualSemverUpdate::Patch,
            kind: VersionChangeKind::Minimum,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_patch_changed() {
        let baseline = "1.0.0";
        let current = "1.0.1";
        let expected = VersionChange {
            level: ActualSemverUpdate::Patch,
            kind: VersionChangeKind::Actual,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_minor_changed() {
        let baseline = "1.0.0";
        let current = "1.1.0";
        let expected = VersionChange {
            level: ActualSemverUpdate::Minor,
            kind: VersionChangeKind::Actual,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_major_changed() {
        let baseline = "0.9.0";
        let current = "1.0.0";
        let expected = VersionChange {
            level: ActualSemverUpdate::Major,
            kind: VersionChangeKind::Actual,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_zerover_minor_changed() {
        let baseline = "0.1.0";
        let current = "0.1.1";
        let expected = VersionChange {
            level: ActualSemverUpdate::Minor,
            kind: VersionChangeKind::Actual,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_zerover_major_changed() {
        let baseline = "0.1.0";
        let current = "0.2.0";
        let expected = VersionChange {
            level: ActualSemverUpdate::Major,
            kind: VersionChangeKind::Actual,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_double_zerover_major_changed() {
        let baseline = "0.0.1";
        let current = "0.0.2";
        let expected = VersionChange {
            level: ActualSemverUpdate::Major,
            kind: VersionChangeKind::Actual,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_pre_same() {
        let baseline = "1.0.0-alpha.0";
        let current = "1.0.0-alpha.0";
        let expected = VersionChange {
            level: ActualSemverUpdate::Major,
            kind: VersionChangeKind::Minimum,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_pre() {
        let baseline = "1.0.0-alpha.0";
        let current = "1.0.0-alpha.1";
        let expected = VersionChange {
            level: ActualSemverUpdate::Major,
            kind: VersionChangeKind::Actual,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_same_version_with_pre() {
        let baseline = "1.0.0-alpha.1";
        let current = "1.0.0";
        let expected = VersionChange {
            level: ActualSemverUpdate::Major,
            kind: VersionChangeKind::Actual,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_minor_changed_with_pre() {
        let baseline = "1.0.0";
        let current = "1.1.0-alpha.1";
        let expected = VersionChange {
            level: ActualSemverUpdate::Minor,
            kind: VersionChangeKind::Actual,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_zerover_same_version() {
        let baseline = "0.1.0";
        let current = "0.1.0";
        let expected = VersionChange {
            level: ActualSemverUpdate::Minor,
            kind: VersionChangeKind::Minimum,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_zerover_zero_same_version() {
        let baseline = "0.0.1";
        let current = "0.0.1";
        let expected = VersionChange {
            level: ActualSemverUpdate::Major,
            kind: VersionChangeKind::Minimum,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_pre_zero_same() {
        let baseline = "0.1.0-alpha.0";
        let current = "0.1.0-alpha.0";
        let expected = VersionChange {
            level: ActualSemverUpdate::Major,
            kind: VersionChangeKind::Minimum,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_build_with_pre_same() {
        let baseline = "1.0.0-alpha.1+build.1";
        let current = "1.0.0-alpha.1+build.2";
        let expected = VersionChange {
            level: ActualSemverUpdate::Major,
            kind: VersionChangeKind::Minimum,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_build_zero_version() {
        let baseline = "0.1.0+build.1";
        let current = "0.1.0+build.2";
        let expected = VersionChange {
            level: ActualSemverUpdate::Minor,
            kind: VersionChangeKind::Minimum,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_ignores_build() {
        let baseline = "1.0.0+hello";
        let current = "1.0.0+world";
        let expected = VersionChange {
            level: ActualSemverUpdate::Patch,
            kind: VersionChangeKind::Minimum,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }
}
