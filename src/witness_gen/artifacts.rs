use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    process::Command,
    sync::{Arc, OnceLock},
};

use anyhow::{Context, Result};
use fs_err as fs;
use serde::Serialize;
use trustfall::{FieldValue, TransparentValue};

use crate::{SemverQuery, WitnessPurpose, util::slugify};

use super::{
    GeneratedWitnessScript, RUN_RESULTS_DIR, RetainedArtifactStatus, RetainedWitnessArtifact,
    TEMP_DIR, USER_REPRODUCTION_DRIVER, WitnessEvaluation, WitnessExecutionClassification,
    WitnessExecutionConfig, WitnessManifestSummary, WitnessRunReport,
};

#[derive(Debug)]
pub(super) struct ScriptExecution {
    pub(super) success: bool,
    pub(super) exit_code: Option<i32>,
    pub(super) stdout: String,
    pub(super) stderr: String,
}

/// Writes the canonical witness script shown to users in retained artifacts.
pub(super) fn write_canonical_witness_script(
    artifact_dir: &Path,
    generated_script: &GeneratedWitnessScript,
) -> Result<()> {
    let witness_path = artifact_dir.join("witness.rs");
    fs::write(&witness_path, &generated_script.witness_rs)
        .with_context(|| format!("failed to write {}", witness_path.display()))?;

    Ok(())
}

/// Writes only the baseline/current script variants that cargo will check.
///
/// The canonical `witness.rs` and query-result manifest are retained-artifact
/// metadata, so the fast path writes just these executable inputs before
/// witness validation and leaves the rest for outcomes that keep artifacts.
pub(super) fn write_executed_script_variants(
    artifact_dir: &Path,
    generated_script: &GeneratedWitnessScript,
) -> Result<()> {
    let baseline_path = artifact_dir.join("baseline.rs");
    fs::write(&baseline_path, &generated_script.baseline_rs)
        .with_context(|| format!("failed to write {}", baseline_path.display()))?;

    let current_path = artifact_dir.join("current.rs");
    fs::write(&current_path, &generated_script.current_rs)
        .with_context(|| format!("failed to write {}", current_path.display()))?;

    Ok(())
}

pub(super) fn write_query_result_file(
    artifact_dir: &Path,
    crate_name: &str,
    semver_query: &SemverQuery,
    purpose: WitnessPurpose,
    result_index: usize,
    generated_script: &GeneratedWitnessScript,
    query_result: &BTreeMap<Arc<str>, FieldValue>,
) -> Result<()> {
    let query_result_manifest = QueryResultManifest {
        format_version: 1,
        crate_name,
        lint_id: semver_query.id.as_str(),
        purpose: purpose.as_str(),
        result_index,
        crate_root_name: &generated_script.crate_root_name,
        baseline_dependency_lines: &generated_script.baseline_dependency.lines,
        current_dependency_lines: &generated_script.current_dependency.lines,
        query_result_json: serde_json::to_string_pretty(&pretty_query_result(query_result))?,
    };

    let query_result_path = artifact_dir.join("query-result.toml");
    fs::write(
        &query_result_path,
        toml::to_string_pretty(&query_result_manifest)?,
    )
    .with_context(|| format!("failed to write {}", query_result_path.display()))?;

    Ok(())
}

fn pretty_query_result(
    query_result: &BTreeMap<Arc<str>, FieldValue>,
) -> BTreeMap<String, TransparentValue> {
    query_result
        .iter()
        .map(|(key, value)| (key.to_string(), value.clone().into()))
        .collect()
}

// Keep this human-readable driver description in sync with the actual command
// assembled in `execute_script()` below.
pub(super) const INTERNAL_DRIVER: &str = "RUSTC_BOOTSTRAP=1 cargo -Zscript check";

pub(super) fn execute_script(
    temp_workspace_root: PathBuf,
    artifact_dir: &Path,
    execution_config: &WitnessExecutionConfig,
    profile_name: &str,
    script_name: &str,
) -> Result<ScriptExecution> {
    let target_dir = temp_workspace_root.join("build-cache").join(profile_name);
    fs::create_dir_all(&target_dir).with_context(|| {
        format!(
            "failed to create witness build cache directory {}",
            target_dir.display()
        )
    })?;

    // Keep this invocation in sync with `INTERNAL_DRIVER` above, which
    // is recorded in retained witness artifact manifests for humans.
    let mut cmd = Command::new("cargo");
    cmd.arg("-Zscript")
        .arg("check")
        .arg("--manifest-path")
        .arg(script_name);
    if let Some(build_target) = execution_config.build_target.as_deref() {
        cmd.arg("--target").arg(build_target);
    }
    // `RUSTC_BOOTSTRAP=1` only unlocks `-Zscript`; it does not make stable Cargo
    // behave like nightly Cargo. Cargo 1.94/1.95 changed the direct
    // `cargo -Zscript <script.rs>` entrypoint to reload config from `CARGO_HOME`
    // only (rust-lang/cargo#14749), and Cargo 1.96 restored script-relative
    // config loading in rust-lang/cargo#16620. Until Rust/Cargo 1.96+, custom
    // rustflags configured in local `.cargo/config.toml` files may therefore be
    // ignored by the direct entrypoint depending on the toolchain version the
    // user is running. Use `cargo -Zscript check --manifest-path <script.rs>`
    // here instead of the direct entrypoint so witness checking does not depend
    // on that version split and can validate cross-target witnesses without
    // trying to run them. Keep the explicit `RUSTFLAGS` plumbing below so the
    // witness sees the same effective build configuration as the semver query,
    // including cfgs that came from rustdoc settings or user config.
    //
    // We also always append `--cap-lints=allow` here. Generated witness scripts
    // already opt into `#![allow(warnings)]`, but user-supplied `-Dwarnings`
    // from Cargo config or environment still wins over crate attributes.
    // Without capping lints at the rustc level, unrelated warnings in the
    // witness can turn into false witness failures.
    let effective_rustflags =
        effective_witness_execution_rustflags(execution_config.rustflags.as_deref());
    cmd.env("RUSTC_BOOTSTRAP", "1")
        .env("CARGO_TERM_COLOR", "never")
        .env("CARGO_TERM_UNICODE", "false")
        .env("CARGO_TARGET_DIR", &target_dir)
        .env("RUSTFLAGS", effective_rustflags.as_ref())
        .env_remove("CARGO_BUILD_RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS");
    let output = cmd
        .current_dir(artifact_dir)
        .output()
        .with_context(|| format!("failed to check witness script `{script_name}`"))?;

    Ok(ScriptExecution {
        success: output.status.success(),
        exit_code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    })
}

pub(super) fn write_execution_logs(
    artifact_dir: &Path,
    baseline: &ScriptExecution,
    current: &ScriptExecution,
) -> Result<()> {
    let baseline_stdout_path = artifact_dir.join("baseline.stdout.txt");
    fs::write(&baseline_stdout_path, &baseline.stdout)
        .with_context(|| format!("failed to write {}", baseline_stdout_path.display()))?;
    let baseline_stderr_path = artifact_dir.join("baseline.stderr.txt");
    fs::write(&baseline_stderr_path, &baseline.stderr)
        .with_context(|| format!("failed to write {}", baseline_stderr_path.display()))?;
    let current_stdout_path = artifact_dir.join("current.stdout.txt");
    fs::write(&current_stdout_path, &current.stdout)
        .with_context(|| format!("failed to write {}", current_stdout_path.display()))?;
    let current_stderr_path = artifact_dir.join("current.stderr.txt");
    fs::write(&current_stderr_path, &current.stderr)
        .with_context(|| format!("failed to write {}", current_stderr_path.display()))?;

    Ok(())
}

pub(super) fn retain_witness_error(
    evaluation: &WitnessEvaluation<'_>,
    artifact_dir: &Path,
    generated_script: Option<&GeneratedWitnessScript>,
    baseline_execution: Option<&ScriptExecution>,
    current_execution: Option<&ScriptExecution>,
    execution_config: Option<&WitnessExecutionConfig>,
    error: Option<anyhow::Error>,
) -> super::SingleWitnessOutcome {
    match generated_script {
        Some(generated_script) => {
            let _ = write_canonical_witness_script(artifact_dir, generated_script);
            let _ = write_query_result_file(
                artifact_dir,
                evaluation.crate_name,
                evaluation.semver_query,
                evaluation.purpose,
                evaluation.result_index,
                generated_script,
                evaluation.query_result,
            );
        }
        None => {
            let query_result_manifest = QueryResultManifest {
                format_version: 1,
                crate_name: evaluation.crate_name,
                lint_id: evaluation.semver_query.id.as_str(),
                purpose: evaluation.purpose.as_str(),
                result_index: evaluation.result_index,
                crate_root_name: "<unavailable>",
                baseline_dependency_lines: &[],
                current_dependency_lines: &[],
                query_result_json: serde_json::to_string_pretty(&pretty_query_result(
                    evaluation.query_result,
                ))
                .expect("failed to serialize query result"),
            };
            let query_result_path = artifact_dir.join("query-result.toml");
            let _ = fs::write(
                &query_result_path,
                toml::to_string_pretty(&query_result_manifest)
                    .expect("failed to serialize query result manifest"),
            );
        }
    }

    let baseline_log = baseline_execution
        .map(|execution| execution.stderr.clone())
        .unwrap_or_else(|| match error.as_ref() {
            Some(error) => format!("baseline witness execution was not completed\n\n{error:#}\n"),
            None => "baseline witness execution was not completed\n".to_owned(),
        });
    let current_log = current_execution
        .map(|execution| execution.stderr.clone())
        .unwrap_or_else(|| match error.as_ref() {
            Some(error) => format!("current witness execution was not completed\n\n{error:#}\n"),
            None => "current witness execution was not completed\n".to_owned(),
        });
    let baseline_stdout = baseline_execution
        .map(|execution| execution.stdout.clone())
        .unwrap_or_default();
    let current_stdout = current_execution
        .map(|execution| execution.stdout.clone())
        .unwrap_or_default();

    let baseline_stdout_path = artifact_dir.join("baseline.stdout.txt");
    let _ = fs::write(&baseline_stdout_path, baseline_stdout);
    let baseline_stderr_path = artifact_dir.join("baseline.stderr.txt");
    let _ = fs::write(&baseline_stderr_path, baseline_log);
    let current_stdout_path = artifact_dir.join("current.stdout.txt");
    let _ = fs::write(&current_stdout_path, current_stdout);
    let current_stderr_path = artifact_dir.join("current.stderr.txt");
    let _ = fs::write(&current_stderr_path, current_log);

    let retained_artifact = RetainedWitnessArtifact {
        lint_id: evaluation.semver_query.id.clone(),
        result_index: evaluation.result_index,
        purpose: evaluation.purpose,
        status: match evaluation.purpose {
            WitnessPurpose::RequiredForCorrectness => RetainedArtifactStatus::RequiredWitnessError,
            WitnessPurpose::ConsistencyCheck => RetainedArtifactStatus::ConsistencyCheckError,
        },
        temp_artifact_dir: artifact_dir.to_path_buf(),
        baseline_exit_code: baseline_execution.and_then(|execution| execution.exit_code),
        current_exit_code: current_execution.and_then(|execution| execution.exit_code),
        execution_config: execution_config.cloned(),
    };
    super::SingleWitnessOutcome::WitnessError {
        retained_artifact: Some(retained_artifact),
    }
}

/// Opportunistically generates explicit lockfiles for retained witness scripts.
///
/// Cargo's embedded-script `-Zlockfile-path` support arrived after Rust 1.93.
/// Local verification showed that Cargo 1.91, 1.92, and 1.93 reject
/// `cargo -Zscript -Zlockfile-path generate-lockfile --manifest-path <script.rs>`
/// even with `RUSTC_BOOTSTRAP=1`, while Cargo 1.94+ accepts it. Callers should
/// use this as an artifact-quality improvement only, not as a required part of
/// witness execution semantics.
///
/// TODO: Make witness lockfiles unconditional once Rust 1.94 is the lowest
/// supported version for cargo-semver-checks.
pub(super) fn maybe_write_script_lockfiles(artifact_dir: &Path) -> Result<bool> {
    if !internal_tool_info().supports_explicit_script_lockfiles {
        return Ok(false);
    }

    write_script_lockfile(artifact_dir, "baseline.rs", "baseline-lockfile/Cargo.lock")?;
    write_script_lockfile(artifact_dir, "current.rs", "current-lockfile/Cargo.lock")?;
    Ok(true)
}

pub(super) fn final_run_root(target_dir: &Path, run_id: &str) -> PathBuf {
    target_dir
        .join(crate::util::SCOPE)
        .join("witnesses")
        .join(RUN_RESULTS_DIR)
        .join(format!("run-{run_id}"))
}

pub(super) fn temp_run_root(target_dir: &Path, run_id: &str) -> PathBuf {
    target_dir
        .join(crate::util::SCOPE)
        .join("witnesses")
        .join(TEMP_DIR)
        .join(format!("run-{run_id}"))
}

pub(super) fn final_artifact_relative_dir(
    crate_name: &str,
    lint_id: &str,
    result_index: usize,
) -> PathBuf {
    PathBuf::from("artifacts")
        .join(slugify(crate_name))
        .join(slugify(lint_id))
        .join(result_index.to_string())
}

/// Serialized manifest for the original lint query result behind one artifact.
#[derive(Serialize)]
struct QueryResultManifest<'a> {
    format_version: u32,
    crate_name: &'a str,
    lint_id: &'a str,
    purpose: &'a str,
    result_index: usize,
    crate_root_name: &'a str,
    baseline_dependency_lines: &'a [String],
    current_dependency_lines: &'a [String],
    query_result_json: String,
}

/// Serialized summary of one finalized retained witness run.
///
/// Written to `run-results/run-<id>/manifest.toml` after retained artifacts
/// have been moved into place.
#[derive(Serialize)]
pub(super) struct RunManifest {
    format_version: u32,
    run_id: String,
    crate_name: String,
    artifact_root: String,
    reproduction_driver: &'static str,
    internal_driver: &'static str,
    internal_rust_version: String,
    internal_cargo_version: String,
    internal_rustc_version: String,
    confirmed_breaking: usize,
    not_confirmed_by_witness: usize,
    consistency_check_mismatches: usize,
    consistency_check_errors: usize,
    required_witness_errors: usize,
    artifacts: Vec<RunManifestArtifact>,
}

/// Serialized manifest entry for one retained artifact.
///
/// `RetainedWitnessArtifact` is the in-memory finalization input: it points at
/// the temp bundle and carries the execution result. This type is the on-disk
/// view after finalization, with relative paths and reproduction commands.
/// File path fields are optional because artifact writes are best-effort in
/// some witness-error paths.
#[derive(Serialize)]
struct RunManifestArtifact {
    crate_name: String,
    lint_id: String,
    purpose: &'static str,
    status: &'static str,
    artifact_dir: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    witness_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    baseline_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    current_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_result_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    baseline_stdout_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    baseline_stderr_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    current_stdout_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    current_stderr_path: Option<String>,
    baseline_exit_code: Option<i32>,
    current_exit_code: Option<i32>,
    build_target: Option<String>,
    effective_rustflags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    baseline_lockfile_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    current_lockfile_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    baseline_internal_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    current_internal_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    baseline_reproduction_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    current_reproduction_command: Option<String>,
}

/// Builds the retained run manifest after all artifacts have reached their
/// final `run-results` locations.
///
/// Callers should invoke this only after moving retained artifacts into their
/// stable directories. The resulting manifest records the exact files users can
/// inspect or rerun, including opportunistic lockfiles when they were
/// generated during finalization. All file paths stored in the manifest are
/// relative to the manifest's directory, so the whole retained run directory
/// can be zipped, moved, and inspected elsewhere without rewriting paths.
pub(super) fn build_run_manifest(
    run_id: &str,
    final_run_root: &Path,
    reports: &[&WitnessRunReport],
) -> Result<RunManifest> {
    let mut crate_names = reports
        .iter()
        .map(|report| report.crate_name.clone())
        .collect::<Vec<_>>();
    crate_names.sort();
    crate_names.dedup();

    let crate_name = if crate_names.len() == 1 {
        crate_names
            .into_iter()
            .next()
            .expect("missing unique crate name")
    } else {
        "<multiple>".to_owned()
    };

    let mut summary = WitnessManifestSummary::default();
    let mut artifacts = Vec::new();
    for report in reports {
        summary.add(&report.manifest_summary);

        for artifact in &report.retained_artifacts {
            let artifact_dir = final_artifact_relative_dir(
                &report.crate_name,
                &artifact.lint_id,
                artifact.result_index,
            );
            let final_artifact_dir = final_run_root.join(&artifact_dir);
            let witness_path = artifact_dir.join("witness.rs");
            let baseline_path = artifact_dir.join("baseline.rs");
            let current_path = artifact_dir.join("current.rs");
            let query_result_path = artifact_dir.join("query-result.toml");
            let baseline_stdout_path = artifact_dir.join("baseline.stdout.txt");
            let baseline_stderr_path = artifact_dir.join("baseline.stderr.txt");
            let current_stdout_path = artifact_dir.join("current.stdout.txt");
            let current_stderr_path = artifact_dir.join("current.stderr.txt");
            let final_witness_path = final_artifact_dir.join("witness.rs");
            let final_baseline_path = final_artifact_dir.join("baseline.rs");
            let final_current_path = final_artifact_dir.join("current.rs");
            let final_query_result_path = final_artifact_dir.join("query-result.toml");
            let final_baseline_stdout_path = final_artifact_dir.join("baseline.stdout.txt");
            let final_baseline_stderr_path = final_artifact_dir.join("baseline.stderr.txt");
            let final_current_stdout_path = final_artifact_dir.join("current.stdout.txt");
            let final_current_stderr_path = final_artifact_dir.join("current.stderr.txt");
            let has_witness_script = final_witness_path.exists();
            let has_baseline_script = final_baseline_path.exists();
            let has_current_script = final_current_path.exists();
            let has_query_result = final_query_result_path.exists();
            let has_baseline_stdout = final_baseline_stdout_path.exists();
            let has_baseline_stderr = final_baseline_stderr_path.exists();
            let has_current_stdout = final_current_stdout_path.exists();
            let has_current_stderr = final_current_stderr_path.exists();

            // Lockfiles are opportunistic: newer Cargo versions retain them for
            // deterministic repro, older ones omit them entirely. Reflect that
            // directly in the manifest by recording lockfile paths only when the
            // files actually exist, and by making the reproduction command
            // prefer the locked variant only in that case.
            let baseline_lockfile_path = lockfile_path_if_present(
                &final_artifact_dir,
                &artifact_dir,
                "baseline-lockfile/Cargo.lock",
            );
            let current_lockfile_path = lockfile_path_if_present(
                &final_artifact_dir,
                &artifact_dir,
                "current-lockfile/Cargo.lock",
            );
            let build_target = artifact
                .execution_config
                .as_ref()
                .and_then(|config| config.build_target.as_deref());
            let effective_rustflags = artifact
                .execution_config
                .as_ref()
                .map(|config| effective_witness_execution_rustflags(config.rustflags.as_deref()));
            artifacts.push(RunManifestArtifact {
                crate_name: report.crate_name.clone(),
                lint_id: artifact.lint_id.clone(),
                purpose: artifact.purpose.as_str(),
                status: artifact.status.as_str(),
                artifact_dir: artifact_dir.to_string_lossy().into_owned(),
                witness_path: has_witness_script
                    .then(|| witness_path.to_string_lossy().into_owned()),
                baseline_path: has_baseline_script
                    .then(|| baseline_path.to_string_lossy().into_owned()),
                current_path: has_current_script
                    .then(|| current_path.to_string_lossy().into_owned()),
                query_result_path: has_query_result
                    .then(|| query_result_path.to_string_lossy().into_owned()),
                baseline_stdout_path: has_baseline_stdout
                    .then(|| baseline_stdout_path.to_string_lossy().into_owned()),
                baseline_stderr_path: has_baseline_stderr
                    .then(|| baseline_stderr_path.to_string_lossy().into_owned()),
                current_stdout_path: has_current_stdout
                    .then(|| current_stdout_path.to_string_lossy().into_owned()),
                current_stderr_path: has_current_stderr
                    .then(|| current_stderr_path.to_string_lossy().into_owned()),
                baseline_exit_code: artifact.baseline_exit_code,
                current_exit_code: artifact.current_exit_code,
                build_target: build_target.map(ToOwned::to_owned),
                effective_rustflags: effective_rustflags.as_ref().map(|x| x.to_string()),
                baseline_lockfile_path: baseline_lockfile_path.clone(),
                current_lockfile_path: current_lockfile_path.clone(),
                baseline_internal_command: has_baseline_script.then(|| {
                    render_command(
                        INTERNAL_DRIVER,
                        &baseline_path,
                        build_target,
                        effective_rustflags.as_deref(),
                    )
                }),
                current_internal_command: has_current_script.then(|| {
                    render_command(
                        INTERNAL_DRIVER,
                        &current_path,
                        build_target,
                        effective_rustflags.as_deref(),
                    )
                }),
                baseline_reproduction_command: has_baseline_script.then(|| {
                    render_reproduction_command(
                        &baseline_path,
                        baseline_lockfile_path.as_deref(),
                        build_target,
                        effective_rustflags.as_deref(),
                    )
                }),
                current_reproduction_command: has_current_script.then(|| {
                    render_reproduction_command(
                        &current_path,
                        current_lockfile_path.as_deref(),
                        build_target,
                        effective_rustflags.as_deref(),
                    )
                }),
            });
        }
    }

    let tool_info = internal_tool_info();

    Ok(RunManifest {
        format_version: 1,
        run_id: run_id.to_owned(),
        crate_name,
        artifact_root: "artifacts".to_owned(),
        reproduction_driver: USER_REPRODUCTION_DRIVER,
        internal_driver: INTERNAL_DRIVER,
        internal_rust_version: tool_info.rust_version.clone(),
        internal_cargo_version: tool_info.cargo_version_output.clone(),
        internal_rustc_version: tool_info.rustc_version_output.clone(),
        confirmed_breaking: summary.confirmed_breaking,
        not_confirmed_by_witness: summary.not_confirmed_by_witness,
        consistency_check_mismatches: summary.consistency_check_mismatches,
        consistency_check_errors: summary.consistency_check_errors,
        required_witness_errors: summary.required_witness_errors,
        artifacts,
    })
}

fn render_command(
    driver: &str,
    script_path: &Path,
    build_target: Option<&str>,
    rustflags: Option<&str>,
) -> String {
    let mut parts = Vec::new();
    if let Some(rustflags) = rustflags {
        parts.push(format!("RUSTFLAGS={}", shell_quote(rustflags)));
    }
    parts.push(driver.to_owned());
    parts.push(String::from("--manifest-path"));
    parts.push(shell_quote(&script_path.to_string_lossy()));
    if let Some(build_target) = build_target {
        parts.push(String::from("--target"));
        parts.push(shell_quote(build_target));
    }
    parts.join(" ")
}

fn render_reproduction_command(
    script_path: &Path,
    lockfile_path: Option<&str>,
    build_target: Option<&str>,
    rustflags: Option<&str>,
) -> String {
    match lockfile_path {
        Some(lockfile_path) => {
            render_locked_reproduction_command(script_path, lockfile_path, build_target, rustflags)
        }
        None => render_command(
            USER_REPRODUCTION_DRIVER,
            script_path,
            build_target,
            rustflags,
        ),
    }
}

fn render_locked_reproduction_command(
    script_path: &Path,
    lockfile_path: &str,
    build_target: Option<&str>,
    rustflags: Option<&str>,
) -> String {
    let mut parts = Vec::new();
    if let Some(rustflags) = rustflags {
        parts.push(format!("RUSTFLAGS={}", shell_quote(rustflags)));
    }
    parts.push(String::from(
        "cargo +nightly -Zscript -Zlockfile-path check",
    ));
    parts.push(String::from("--manifest-path"));
    parts.push(shell_quote(&script_path.to_string_lossy()));
    parts.push(String::from("--config"));
    parts.push(shell_quote(&lockfile_config_arg(Path::new(lockfile_path))));
    if let Some(build_target) = build_target {
        parts.push(String::from("--target"));
        parts.push(shell_quote(build_target));
    }
    parts.push(String::from("--locked"));
    parts.join(" ")
}

/// Add `--cap-lints=allow` to the given rustflags, if any.
fn effective_witness_execution_rustflags(
    rustflags: Option<&str>,
) -> std::borrow::Cow<'static, str> {
    match rustflags {
        Some(rustflags) => {
            let mut rustflags = rustflags.to_owned();
            rustflags.push_str(" --cap-lints=allow");
            std::borrow::Cow::Owned(rustflags)
        }
        None => std::borrow::Cow::Borrowed("--cap-lints=allow"),
    }
}

#[derive(Debug)]
struct InternalToolInfo {
    cargo_version_output: String,
    rust_version: String,
    rustc_version_output: String,
    supports_explicit_script_lockfiles: bool,
}

fn internal_tool_info() -> &'static InternalToolInfo {
    static TOOL_INFO: OnceLock<InternalToolInfo> = OnceLock::new();
    TOOL_INFO.get_or_init(detect_internal_tool_info)
}

fn detect_internal_tool_info() -> InternalToolInfo {
    let cargo_version_output = tool_version("cargo");
    let rustc_version_output = tool_version("rustc");

    InternalToolInfo {
        supports_explicit_script_lockfiles: supports_explicit_script_lockfiles_from_output(
            &cargo_version_output,
        ),
        rust_version: parsed_tool_release(&rustc_version_output)
            .unwrap_or("<unavailable>")
            .to_owned(),
        cargo_version_output,
        rustc_version_output,
    }
}

#[cfg(test)]
pub(super) fn supports_explicit_script_lockfiles() -> bool {
    internal_tool_info().supports_explicit_script_lockfiles
}

pub(super) fn supports_explicit_script_lockfiles_from_output(version_output: &str) -> bool {
    let Some(release) = parsed_tool_release(version_output) else {
        return false;
    };

    let mut parts = release.split('.');
    let major = parts.next().and_then(|value| value.parse::<u64>().ok());
    let minor = parts.next().and_then(|value| value.parse::<u64>().ok());

    matches!((major, minor), (Some(major), Some(minor)) if major > 1 || (major == 1 && minor >= 94))
}

fn tool_version(tool: &str) -> String {
    Command::new(tool)
        .arg("--version")
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .filter(|version| !version.is_empty())
        .unwrap_or_else(|| format!("{tool} <unavailable>"))
}

fn parsed_tool_release(version_output: &str) -> Option<&str> {
    version_output
        .split_whitespace()
        .nth(1)
        .filter(|release| !release.is_empty())
}

fn write_script_lockfile(
    artifact_dir: &Path,
    script_name: &str,
    lockfile_relative_path: &str,
) -> Result<()> {
    let lockfile_path = artifact_dir.join(lockfile_relative_path);
    if let Some(parent) = lockfile_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "failed to create witness lockfile directory {}",
                parent.display()
            )
        })?;
    }
    let output = Command::new("cargo")
        .arg("-Zscript")
        .arg("-Zlockfile-path")
        .arg("generate-lockfile")
        .arg("--manifest-path")
        .arg(script_name)
        .arg("--config")
        .arg(lockfile_config_arg(&lockfile_path))
        .arg("--offline")
        .current_dir(artifact_dir)
        .env("RUSTC_BOOTSTRAP", "1")
        .env("CARGO_TERM_COLOR", "never")
        .env("CARGO_TERM_UNICODE", "false")
        .output()
        .with_context(|| {
            format!(
                "failed to generate witness lockfile `{}` for `{script_name}`",
                lockfile_path.display(),
            )
        })?;

    if output.status.success() {
        return Ok(());
    }

    anyhow::bail!(
        "failed to generate witness lockfile `{}` for `{script_name}`\n\
         stdout:\n{}\n\
         stderr:\n{}",
        lockfile_path.display(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

fn lockfile_config_arg(lockfile_path: &Path) -> String {
    format!(
        "resolver.lockfile-path={}",
        toml::Value::String(lockfile_path.to_string_lossy().into_owned()),
    )
}

fn lockfile_path_if_present(
    final_artifact_dir: &Path,
    artifact_dir: &Path,
    file_name: &str,
) -> Option<String> {
    let lockfile_path = final_artifact_dir.join(file_name);
    lockfile_path
        .exists()
        .then(|| artifact_dir.join(file_name).to_string_lossy().into_owned())
}

fn shell_quote(value: &str) -> String {
    let escaped = value.replace('\'', "'\"'\"'");
    format!("'{escaped}'")
}

pub(super) fn classify_execution_outcome(
    baseline_success: bool,
    current_success: bool,
) -> WitnessExecutionClassification {
    match (baseline_success, current_success) {
        (true, false) => WitnessExecutionClassification::ConfirmedBreaking,
        (true, true) => WitnessExecutionClassification::NotConfirmedByWitness,
        _ => WitnessExecutionClassification::WitnessError,
    }
}
