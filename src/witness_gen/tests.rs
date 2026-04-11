use std::{borrow::Cow, collections::BTreeMap, path::Path};

use tame_index::IndexVersion;

use super::{
    artifacts::{
        INTERNAL_DRIVER, maybe_write_script_lockfiles, supports_explicit_script_lockfiles,
        supports_explicit_script_lockfiles_from_output,
    },
    test_support::*,
    *,
};

#[test]
fn dependency_blocks_toggle_between_script_variants() {
    let baseline = DependencyProfile {
        lines: vec![String::from(
            "upstream_dep = { package = \"upstream-dep\", version = \"=1.2.3\" }",
        )],
    };
    let current = DependencyProfile {
        lines: vec![String::from(
            "upstream_dep = { package = \"upstream-dep\", path = \"/tmp/current-upstream\" }",
        )],
    };
    let scripts =
        render_script_variants("upstream_dep", &baseline, &current, "pub fn witness() {}");

    assert!(
        scripts
            .witness_rs
            .contains("upstream_dep = { package = \"upstream-dep\", version = \"=1.2.3\" }")
    );
    assert!(scripts.witness_rs.contains(
        "# upstream_dep = { package = \"upstream-dep\", path = \"/tmp/current-upstream\" }"
    ));
    assert!(
        scripts
            .current_rs
            .contains("# upstream_dep = { package = \"upstream-dep\", version = \"=1.2.3\" }")
    );
    assert!(scripts.current_rs.contains(
        "upstream_dep = { package = \"upstream-dep\", path = \"/tmp/current-upstream\" }"
    ));
}

#[test]
fn exact_version_and_local_path_dependencies_render_correctly() {
    let temp_dir = TempDir::new("cargo-semver-checks-dep-render");
    let manifest = make_local_manifest(
        &temp_dir,
        "crate",
        "demo-package",
        Some("renamed_lib"),
        "",
        "pub fn removed() {}",
    );
    let local_request =
        make_request_with_settings(&manifest, false, false, &["witness_feature"], None);
    let local_line = dependency_profile(&local_request, "renamed_lib")
        .expect("failed to render local dependency")
        .lines
        .into_iter()
        .next()
        .expect("missing local dependency line");

    let version = IndexVersion::fake("demo-package", "1.2.3");
    let registry_request = CrateDataRequest::from_index(
        &version,
        false,
        [Cow::Borrowed("witness_feature")].into_iter().collect(),
        None,
        true,
    );
    let registry_line = dependency_profile(&registry_request, "demo_package")
        .expect("failed to render registry dependency")
        .lines
        .into_iter()
        .next()
        .expect("missing registry dependency line");

    assert!(registry_line.contains("version = \"=1.2.3\""));
    assert!(registry_line.contains("default-features = false"));
    assert!(registry_line.contains("features = [\"witness_feature\"]"));
    assert!(local_line.contains("path = "));
    assert!(local_line.contains("package = \"demo-package\""));
    assert!(local_line.contains("default-features = false"));
    assert!(local_line.contains("features = [\"witness_feature\"]"));
}

#[test]
fn crate_root_name_uses_query_path_or_manifest_fallback() {
    let temp_dir = TempDir::new("cargo-semver-checks-root-name");
    let manifest = make_local_manifest(
        &temp_dir,
        "crate",
        "demo-package",
        Some("renamed_lib"),
        "",
        "pub fn removed() {}",
    );
    let request = make_request(&manifest, true);

    let query_root =
        determine_crate_root_name(&make_query_result(&["renamed_lib", "removed"]), &request)
            .expect("failed to read crate root name from query result");
    assert_eq!(query_root, "renamed_lib");

    let fallback_root = determine_crate_root_name(&BTreeMap::new(), &request)
        .expect("failed to use manifest fallback");
    assert_eq!(fallback_root, "renamed_lib");
}

#[test]
fn feature_gated_witness_preserves_selected_features() {
    let temp_dir = TempDir::new("cargo-semver-checks-feature-witness");
    let manifest_extra = "\n[features]\nwitness_feature = []\n";
    let baseline_manifest = make_local_manifest(
        &temp_dir,
        "baseline",
        "demo-package",
        Some("renamed_lib"),
        manifest_extra,
        "#[cfg(feature = \"witness_feature\")]\npub fn removed() {}\n",
    );
    let current_manifest = make_local_manifest(
        &temp_dir,
        "current",
        "demo-package",
        Some("renamed_lib"),
        manifest_extra,
        "",
    );
    let baseline_request =
        make_request_with_settings(&baseline_manifest, true, false, &["witness_feature"], None);
    let current_request =
        make_request_with_settings(&current_manifest, false, false, &["witness_feature"], None);

    let (report, lint_results, _config) = run_witness_with_requests(
        WitnessPurpose::RequiredForCorrectness,
        false,
        "demo-package",
        &baseline_request,
        &current_request,
        temp_dir.path().join("target"),
    );

    assert!(report.statistics.is_none());
    assert_eq!(lint_results[0].query_results.len(), 1);
    assert!(report.retained_artifacts.is_empty());
}

#[test]
fn cfg_gated_witness_executes_with_cfg_rustflags() {
    let temp_dir = TempDir::new("cargo-semver-checks-cfg-witness");
    let baseline_manifest = make_local_manifest(
        &temp_dir,
        "baseline",
        "demo-package",
        Some("renamed_lib"),
        "",
        "#[cfg(custom)]\npub fn removed() {}\n",
    );
    let current_manifest = make_local_manifest(
        &temp_dir,
        "current",
        "demo-package",
        Some("renamed_lib"),
        "",
        "",
    );
    let baseline_request = make_request(&baseline_manifest, true);
    let current_request = make_request(&current_manifest, false);
    let witness_data = WitnessGenerationData::new(
        Some(&baseline_request),
        Some(&current_request),
        temp_dir.path().join("target"),
    );
    let semver_query = make_semver_query(WitnessPurpose::RequiredForCorrectness);
    let query_result = make_query_result(&["renamed_lib", "removed"]);
    let mut config = GlobalConfig::new();
    let crate_root_name = determine_crate_root_name(&query_result, &baseline_request)
        .expect("failed to determine cfg witness crate root name");
    let baseline_dependency = dependency_profile(&baseline_request, &crate_root_name)
        .expect("failed to render cfg witness baseline dependency");
    let current_dependency = dependency_profile(&current_request, &crate_root_name)
        .expect("failed to render cfg witness current dependency");
    let witness_body = format!("pub fn witness() {{ {crate_root_name}::removed(); }}");
    let generated_script = render_script_variants(
        &crate_root_name,
        &baseline_dependency,
        &current_dependency,
        &witness_body,
    );
    let temp_artifact_dir = temp_dir.path().join("artifacts").join("cfg");
    fs::create_dir_all(&temp_artifact_dir).expect("failed to create cfg witness artifact dir");
    let run_id = config.run_id().to_owned();

    let outcome = evaluate_generated_witness(
        &mut config,
        GeneratedWitnessRun {
            temp_artifact_dir: &temp_artifact_dir,
            witness_target_dir: witness_data.target_dir.as_path(),
            run_id: &run_id,
            execution_config: WitnessExecutionConfig {
                build_target: None,
                rustflags: Some("--cfg custom".to_owned()),
            },
            generated_script,
        },
        WitnessEvaluation {
            crate_name: "demo-package",
            semver_query: &semver_query,
            purpose: WitnessPurpose::RequiredForCorrectness,
            result_index: 0,
            query_result: &query_result,
        },
    );

    match outcome {
        SingleWitnessOutcome::ConfirmedBreaking => {
            // Expected outcome. Other paths lead to panic.
        }
        SingleWitnessOutcome::NotConfirmedByWitness { .. } => {
            panic!("cfg-gated witness unexpectedly compiled on current")
        }
        SingleWitnessOutcome::WitnessError { retained_artifact } => {
            let retained_artifact =
                retained_artifact.expect("cfg-gated witness error should retain an artifact");
            let baseline_stderr = fs::read_to_string(
                retained_artifact
                    .temp_artifact_dir
                    .join("baseline.stderr.txt"),
            )
            .expect("failed to read cfg witness baseline stderr");
            let current_stderr = fs::read_to_string(
                retained_artifact
                    .temp_artifact_dir
                    .join("current.stderr.txt"),
            )
            .expect("failed to read cfg witness current stderr");
            panic!(
                "cfg-gated witness errored\n\
                 baseline stderr:\n{baseline_stderr}\n\
                 current stderr:\n{current_stderr}"
            );
        }
    }
}

#[test]
fn explicit_script_lockfiles_are_supported_from_cargo_194() {
    assert!(!supports_explicit_script_lockfiles_from_output(
        "cargo 1.91.0 (123456789 2025-10-30)"
    ));
    assert!(!supports_explicit_script_lockfiles_from_output(
        "cargo 1.92.0 (123456789 2025-11-27)"
    ));
    assert!(!supports_explicit_script_lockfiles_from_output(
        "cargo 1.93.0 (123456789 2025-12-18)"
    ));
    assert!(supports_explicit_script_lockfiles_from_output(
        "cargo 1.94.0 (85eff7c80 2026-01-15)"
    ));
    assert!(supports_explicit_script_lockfiles_from_output(
        "cargo 1.96.0-nightly (f298b8c82 2026-02-24)"
    ));
}

#[test]
fn maybe_write_script_lockfiles_only_writes_when_supported() {
    let temp_dir = TempDir::new("cargo-semver-checks-script-lockfiles");
    let artifact_dir = temp_dir.path().join("artifacts").join("lockfiles");

    write_file(
        &artifact_dir.join("dep/Cargo.toml"),
        "\
[package]
name = \"witness_dep\"
version = \"0.1.0\"
edition = \"2024\"
",
    );
    write_file(
        &artifact_dir.join("dep/src/lib.rs"),
        "\
pub fn reachable() {}
",
    );
    write_file(
        &artifact_dir.join("baseline.rs"),
        "\
#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = \"2024\"

[dependencies]
witness_dep = { path = \"dep\" }
---
fn main() {
    witness_dep::reachable();
}
",
    );
    write_file(
        &artifact_dir.join("current.rs"),
        "\
#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = \"2024\"

[dependencies]
witness_dep = { path = \"dep\" }
---
fn main() {
    witness_dep::reachable();
}
",
    );

    let wrote_lockfiles = maybe_write_script_lockfiles(&artifact_dir)
        .expect("failed to conditionally write witness script lockfiles");
    let baseline_lockfile = artifact_dir.join("baseline-lockfile/Cargo.lock");
    let current_lockfile = artifact_dir.join("current-lockfile/Cargo.lock");

    assert_eq!(wrote_lockfiles, supports_explicit_script_lockfiles());
    assert_eq!(
        baseline_lockfile.exists(),
        supports_explicit_script_lockfiles()
    );
    assert_eq!(
        current_lockfile.exists(),
        supports_explicit_script_lockfiles()
    );
}

#[test]
fn execute_script_caps_warnings_even_when_rustflags_deny_them() {
    let temp_dir = TempDir::new("cargo-semver-checks-cap-lints");
    let temp_workspace_root = temp_dir.path().join("workspace");
    let artifact_dir = temp_dir.path().join("artifacts").join("warning-script");
    fs::create_dir_all(&artifact_dir).expect("failed to create warning-script artifact dir");
    write_file(
        &artifact_dir.join("script.rs"),
        "\
#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = \"2024\"
---
#![allow(warnings)]

fn main() {
    let unused = 0;
}
",
    );

    let execution = execute_script(
        temp_workspace_root,
        &artifact_dir,
        &WitnessExecutionConfig {
            build_target: None,
            rustflags: Some("-Dwarnings".to_owned()),
        },
        "warning-profile",
        "script.rs",
    )
    .expect("failed to execute witness script with deny-warnings rustflags");

    assert!(
        execution.success,
        "expected witness execution to cap lints even when rustflags deny \
         warnings\nstdout:\n{}\nstderr:\n{}",
        execution.stdout, execution.stderr,
    );
}

// `execute_script_caps_warnings_even_when_rustflags_deny_them()` only covers
// warnings in the generated witness script itself.
// This one guards against a subtler regression: if witness execution ever moves
// to a mechanism like `cargo rustc -- --cap-lints=allow`, lint capping would
// only apply to the top-level script crate and a warning in the baseline/current
// dependency could still make the witness fail for an unrelated reason.
#[test]
fn execute_script_caps_dependency_warnings_even_when_rustflags_deny_them() {
    let temp_dir = TempDir::new("cargo-semver-checks-cap-lints-dep");
    let temp_workspace_root = temp_dir.path().join("workspace");
    let artifact_dir = temp_dir
        .path()
        .join("artifacts")
        .join("warning-dependency-script");
    let dependency_dir = artifact_dir.join("warning-dependency");
    fs::create_dir_all(&dependency_dir).expect("failed to create warning-dependency dir");
    write_file(
        &dependency_dir.join("Cargo.toml"),
        "\
[package]
name = \"warning_dependency\"
version = \"0.1.0\"
edition = \"2024\"
",
    );
    write_file(
        &dependency_dir.join("src/lib.rs"),
        "\
pub fn dependency_warning() {
    let unused = 0;
}
",
    );
    fs::create_dir_all(&artifact_dir).expect("failed to create warning-dependency artifact dir");
    write_file(
        &artifact_dir.join("script.rs"),
        "\
#!/usr/bin/env -S cargo +nightly -Zscript
---
package.edition = \"2024\"

[dependencies]
warning_dependency = { path = \"warning-dependency\" }
---
#![allow(warnings)]

fn main() {
    warning_dependency::dependency_warning();
}
",
    );

    let execution = execute_script(
        temp_workspace_root,
        &artifact_dir,
        &WitnessExecutionConfig {
            build_target: None,
            rustflags: Some("-Dwarnings".to_owned()),
        },
        "warning-dependency-profile",
        "script.rs",
    )
    .expect("failed to execute witness script with warning dependency");

    assert!(
        execution.success,
        "expected witness execution to cap dependency warnings even when \
         rustflags deny warnings\nstdout:\n{}\nstderr:\n{}",
        execution.stdout, execution.stderr,
    );
}

#[test]
fn confirmed_breakage_retains_no_artifacts() {
    let (report, lint_results, temp_dir, config) = run_smoke_witness(
        WitnessPurpose::RequiredForCorrectness,
        false,
        "pub fn removed() {}",
        "",
    );

    assert!(report.statistics.is_none());
    assert_eq!(lint_results[0].query_results.len(), 1);
    assert!(report.retained_artifacts.is_empty());

    let finalized = finalize_retained_artifacts(config.run_id(), &[report])
        .expect("failed to finalize retained witness artifacts");
    assert!(finalized.is_empty());
    assert!(!final_run_root(&temp_dir.path().join("target"), config.run_id()).exists());
}

#[test]
fn consistency_check_mismatch_retains_artifacts_without_suppressing_lint() {
    let (report, lint_results, temp_dir, config) = run_smoke_witness(
        WitnessPurpose::ConsistencyCheck,
        true,
        "pub fn removed() {}",
        "pub fn removed() {}",
    );

    let statistics = report
        .statistics
        .as_ref()
        .expect("expected witness statistics");
    assert_eq!(statistics.consistency_check_mismatches(), 1);
    assert_eq!(lint_results[0].query_results.len(), 1);
    assert_eq!(report.retained_artifacts.len(), 1);

    let finalized = finalize_retained_artifacts(config.run_id(), &[report])
        .expect("failed to finalize retained witness artifacts");
    assert_eq!(finalized.len(), 1);
    let manifest = fs::read_to_string(finalized[0].join("manifest.toml"))
        .expect("failed to read retained artifact manifest");
    assert!(manifest.contains("status = \"consistency_check_mismatch\""));
    assert!(manifest.contains("consistency_check_mismatches = 1"));
    assert!(
        finalized[0]
            .join("artifacts/demo_package/witness_test/0/witness.rs")
            .exists()
    );
    assert!(final_run_root(&temp_dir.path().join("target"), config.run_id()).exists());
}

#[test]
fn required_witness_not_confirmed_suppresses_lint_and_retains_artifacts() {
    let (report, lint_results, _temp_dir, config) = run_smoke_witness(
        WitnessPurpose::RequiredForCorrectness,
        false,
        "pub fn removed() {}",
        "pub fn removed() {}",
    );

    let statistics = report
        .statistics
        .as_ref()
        .expect("expected witness statistics");
    assert_eq!(statistics.not_confirmed_by_witness(), 1);
    assert_eq!(lint_results[0].query_results.len(), 0);
    assert_eq!(report.retained_artifacts.len(), 1);

    let finalized = finalize_retained_artifacts(config.run_id(), &[report])
        .expect("failed to finalize retained witness artifacts");
    let manifest = fs::read_to_string(finalized[0].join("manifest.toml"))
        .expect("failed to read retained artifact manifest");
    assert!(manifest.contains("status = \"not_confirmed_by_witness\""));
    assert!(manifest.contains("not_confirmed_by_witness = 1"));
    let query_result_manifest = fs::read_to_string(
        finalized[0].join("artifacts/demo_package/witness_test/0/query-result.toml"),
    )
    .expect("failed to read query result manifest");
    let query_result_manifest: toml::Value =
        toml::from_str(&query_result_manifest).expect("failed to parse query result manifest");
    let query_result_manifest = query_result_manifest
        .as_table()
        .expect("query result manifest should serialize as a TOML table");
    assert_eq!(
        query_result_manifest
            .get("crate_root_name")
            .and_then(toml::Value::as_str),
        Some("renamed_lib")
    );
}

#[test]
fn required_witness_error_retains_artifacts_and_marks_error() {
    let (report, lint_results, _temp_dir, config) = run_smoke_witness(
        WitnessPurpose::RequiredForCorrectness,
        false,
        "",
        "pub fn removed() {}",
    );

    let statistics = report
        .statistics
        .as_ref()
        .expect("expected witness statistics");
    assert_eq!(statistics.required_witness_errors(), 1);
    assert_eq!(lint_results[0].query_results.len(), 0);
    assert_eq!(report.retained_artifacts.len(), 1);

    let finalized = finalize_retained_artifacts(config.run_id(), &[report])
        .expect("failed to finalize retained witness artifacts");
    let manifest = fs::read_to_string(finalized[0].join("manifest.toml"))
        .expect("failed to read retained artifact manifest");
    assert!(manifest.contains("status = \"required_witness_error\""));
    assert!(manifest.contains("required_witness_errors = 1"));
}

#[test]
fn run_manifest_records_execution_context() {
    let temp_dir = TempDir::new("cargo-semver-checks-run-manifest");
    let final_run_root = temp_dir.path().join("run-results").join("run-abc123");
    let final_artifact_dir = final_run_root.join("artifacts/demo_package/witness_test/0");
    fs::create_dir_all(&final_artifact_dir).expect("failed to create final artifact dir");
    write_file(
        &final_artifact_dir.join("baseline-lockfile/Cargo.lock"),
        "\
version = 4
",
    );
    write_file(
        &final_artifact_dir.join("current-lockfile/Cargo.lock"),
        "\
version = 4
",
    );
    write_file(&final_artifact_dir.join("witness.rs"), "fn main() {}\n");
    write_file(&final_artifact_dir.join("baseline.rs"), "fn main() {}\n");
    write_file(&final_artifact_dir.join("current.rs"), "fn main() {}\n");
    write_file(&final_artifact_dir.join("query-result.toml"), "\n");
    write_file(&final_artifact_dir.join("baseline.stdout.txt"), "\n");
    write_file(&final_artifact_dir.join("baseline.stderr.txt"), "\n");
    write_file(&final_artifact_dir.join("current.stdout.txt"), "\n");
    write_file(&final_artifact_dir.join("current.stderr.txt"), "\n");

    let report = WitnessRunReport {
        crate_name: "demo-package".to_owned(),
        target_dir: temp_dir.path().join("target"),
        statistics: None,
        manifest_summary: WitnessManifestSummary {
            confirmed_breaking: 0,
            not_confirmed_by_witness: 1,
            consistency_check_mismatches: 0,
            consistency_check_errors: 0,
            required_witness_errors: 0,
        },
        retained_artifacts: vec![RetainedWitnessArtifact {
            lint_id: "witness_test".to_owned(),
            result_index: 0,
            purpose: WitnessPurpose::RequiredForCorrectness,
            status: RetainedArtifactStatus::NotConfirmedByWitness,
            temp_artifact_dir: temp_dir.path().join("tmp-artifact"),
            baseline_exit_code: Some(0),
            current_exit_code: Some(0),
            execution_config: Some(WitnessExecutionConfig {
                build_target: Some("wasm32-unknown-unknown".to_owned()),
                rustflags: Some("--cfg custom".to_owned()),
            }),
        }],
    };

    let manifest = build_run_manifest("abc123", &final_run_root, &[&report])
        .expect("failed to build run manifest");
    let manifest_text =
        toml::to_string_pretty(&manifest).expect("failed to serialize run manifest");
    let final_run_root_text = final_run_root.to_string_lossy();
    assert!(
        !manifest_text.contains(final_run_root_text.as_ref()),
        "run manifest should not contain paths rooted at {}:\n{manifest_text}",
        final_run_root.display(),
    );
    let manifest_value: toml::Value =
        toml::from_str(&manifest_text).expect("failed to parse serialized run manifest");
    let manifest_table = manifest_value
        .as_table()
        .expect("run manifest should serialize as a TOML table");

    assert_eq!(
        manifest_table
            .get("reproduction_driver")
            .and_then(toml::Value::as_str),
        Some(USER_REPRODUCTION_DRIVER)
    );
    assert_eq!(
        manifest_table
            .get("internal_driver")
            .and_then(toml::Value::as_str),
        Some(INTERNAL_DRIVER)
    );

    let artifacts = manifest_table
        .get("artifacts")
        .and_then(toml::Value::as_array)
        .expect("manifest should contain an `artifacts` array");
    assert_eq!(artifacts.len(), 1);
    let artifact = artifacts[0]
        .as_table()
        .expect("artifact entry should serialize as a TOML table");

    assert_eq!(
        artifact.get("status").and_then(toml::Value::as_str),
        Some("not_confirmed_by_witness")
    );
    assert_eq!(
        artifact.get("purpose").and_then(toml::Value::as_str),
        Some("required_for_correctness")
    );
    assert_eq!(
        artifact.get("build_target").and_then(toml::Value::as_str),
        Some("wasm32-unknown-unknown")
    );
    assert_eq!(
        artifact
            .get("effective_rustflags")
            .and_then(toml::Value::as_str),
        Some("--cfg custom --cap-lints=allow")
    );
    assert_eq!(
        artifact
            .get("baseline_lockfile_path")
            .and_then(toml::Value::as_str),
        Some("artifacts/demo_package/witness_test/0/baseline-lockfile/Cargo.lock")
    );
    assert_eq!(
        artifact
            .get("current_lockfile_path")
            .and_then(toml::Value::as_str),
        Some("artifacts/demo_package/witness_test/0/current-lockfile/Cargo.lock")
    );
    let artifact_root = manifest_table
        .get("artifact_root")
        .and_then(toml::Value::as_str)
        .expect("missing artifact_root");
    assert!(!Path::new(artifact_root).is_absolute());
    assert!(final_run_root.join(artifact_root).exists());

    for key in [
        "artifact_dir",
        "baseline_lockfile_path",
        "baseline_path",
        "baseline_stderr_path",
        "baseline_stdout_path",
        "current_lockfile_path",
        "current_path",
        "current_stderr_path",
        "current_stdout_path",
        "query_result_path",
        "witness_path",
    ] {
        let path = artifact
            .get(key)
            .and_then(toml::Value::as_str)
            .unwrap_or_else(|| panic!("missing path field `{key}`"));
        assert!(
            !Path::new(path).is_absolute(),
            "manifest path field `{key}` should be relative, got `{path}`",
        );
        assert!(
            final_run_root.join(path).exists(),
            "manifest path field `{key}` should resolve relative to {}, got `{path}`",
            final_run_root.display(),
        );
    }

    let baseline_internal_command = artifact
        .get("baseline_internal_command")
        .and_then(toml::Value::as_str)
        .expect("missing baseline_internal_command");
    assert!(baseline_internal_command.contains("RUSTFLAGS='--cfg custom --cap-lints=allow'"));
    assert!(baseline_internal_command.contains("cargo -Zscript check --manifest-path"));
    assert!(baseline_internal_command.contains("--target 'wasm32-unknown-unknown'"));

    let baseline_reproduction_command = artifact
        .get("baseline_reproduction_command")
        .and_then(toml::Value::as_str)
        .expect("missing baseline_reproduction_command");
    assert!(baseline_reproduction_command.contains("RUSTFLAGS='--cfg custom --cap-lints=allow'"));
    assert!(
        baseline_reproduction_command
            .contains("cargo +nightly -Zscript -Zlockfile-path check --manifest-path")
    );
    assert!(baseline_reproduction_command.contains("--target 'wasm32-unknown-unknown'"));
    assert!(baseline_reproduction_command.contains(concat!(
        "--config 'resolver.lockfile-path=\"",
        "artifacts/demo_package/witness_test/0/baseline-lockfile/Cargo.lock\"'"
    )));
    assert!(baseline_reproduction_command.contains("--locked"));
}

#[test]
fn run_manifest_omits_lockfile_fields_when_absent() {
    let temp_dir = TempDir::new("cargo-semver-checks-run-manifest-omits-lockfiles");
    let final_run_root = temp_dir.path().join("run-results").join("run-abc123");
    let final_artifact_dir = final_run_root.join("artifacts/demo_package/witness_test/0");
    fs::create_dir_all(&final_artifact_dir).expect("failed to create final artifact dir");
    write_file(&final_artifact_dir.join("witness.rs"), "fn main() {}\n");
    write_file(&final_artifact_dir.join("baseline.rs"), "fn main() {}\n");
    write_file(&final_artifact_dir.join("current.rs"), "fn main() {}\n");
    write_file(&final_artifact_dir.join("query-result.toml"), "\n");
    write_file(&final_artifact_dir.join("baseline.stdout.txt"), "\n");
    write_file(&final_artifact_dir.join("baseline.stderr.txt"), "\n");
    write_file(&final_artifact_dir.join("current.stdout.txt"), "\n");
    write_file(&final_artifact_dir.join("current.stderr.txt"), "\n");

    let report = WitnessRunReport {
        crate_name: "demo-package".to_owned(),
        target_dir: temp_dir.path().join("target"),
        statistics: None,
        manifest_summary: WitnessManifestSummary {
            confirmed_breaking: 0,
            not_confirmed_by_witness: 1,
            consistency_check_mismatches: 0,
            consistency_check_errors: 0,
            required_witness_errors: 0,
        },
        retained_artifacts: vec![RetainedWitnessArtifact {
            lint_id: "witness_test".to_owned(),
            result_index: 0,
            purpose: WitnessPurpose::RequiredForCorrectness,
            status: RetainedArtifactStatus::NotConfirmedByWitness,
            temp_artifact_dir: temp_dir.path().join("tmp-artifact"),
            baseline_exit_code: Some(0),
            current_exit_code: Some(0),
            execution_config: Some(WitnessExecutionConfig {
                build_target: Some("wasm32-unknown-unknown".to_owned()),
                rustflags: Some("--cfg custom".to_owned()),
            }),
        }],
    };

    let manifest = build_run_manifest("abc123", &final_run_root, &[&report])
        .expect("failed to build run manifest");
    let manifest_text =
        toml::to_string_pretty(&manifest).expect("failed to serialize run manifest");
    let manifest_value: toml::Value =
        toml::from_str(&manifest_text).expect("failed to parse serialized run manifest");
    let artifacts = manifest_value
        .as_table()
        .expect("run manifest should serialize as a TOML table")
        .get("artifacts")
        .and_then(toml::Value::as_array)
        .expect("manifest should contain an `artifacts` array");
    let artifact = artifacts[0]
        .as_table()
        .expect("artifact entry should serialize as a TOML table");
    let baseline_reproduction_command = artifact
        .get("baseline_reproduction_command")
        .and_then(toml::Value::as_str)
        .expect("missing baseline_reproduction_command");
    assert!(
        baseline_reproduction_command.contains("cargo +nightly -Zscript check --manifest-path")
    );
    assert!(!baseline_reproduction_command.contains("-Zlockfile-path"));
    assert!(!artifact.contains_key("baseline_lockfile_path"));
    assert!(!artifact.contains_key("current_lockfile_path"));
}

#[test]
fn run_manifest_omits_file_fields_when_files_are_absent() {
    let temp_dir = TempDir::new("cargo-semver-checks-run-manifest-omits-files");
    let final_run_root = temp_dir.path().join("run-results").join("run-abc123");
    let report = WitnessRunReport {
        crate_name: "demo-package".to_owned(),
        target_dir: temp_dir.path().join("target"),
        statistics: None,
        manifest_summary: WitnessManifestSummary {
            confirmed_breaking: 0,
            not_confirmed_by_witness: 0,
            consistency_check_mismatches: 0,
            consistency_check_errors: 0,
            required_witness_errors: 1,
        },
        retained_artifacts: vec![RetainedWitnessArtifact {
            lint_id: "witness_test".to_owned(),
            result_index: 0,
            purpose: WitnessPurpose::RequiredForCorrectness,
            status: RetainedArtifactStatus::RequiredWitnessError,
            temp_artifact_dir: temp_dir.path().join("tmp-artifact"),
            baseline_exit_code: None,
            current_exit_code: None,
            execution_config: Some(WitnessExecutionConfig {
                build_target: None,
                rustflags: None,
            }),
        }],
    };

    let manifest = build_run_manifest("abc123", &final_run_root, &[&report])
        .expect("failed to build run manifest");
    let manifest_text =
        toml::to_string_pretty(&manifest).expect("failed to serialize run manifest");
    let manifest_value: toml::Value =
        toml::from_str(&manifest_text).expect("failed to parse serialized run manifest");
    let artifacts = manifest_value
        .as_table()
        .expect("run manifest should serialize as a TOML table")
        .get("artifacts")
        .and_then(toml::Value::as_array)
        .expect("manifest should contain an `artifacts` array");
    let artifact = artifacts[0]
        .as_table()
        .expect("artifact entry should serialize as a TOML table");

    for key in [
        "baseline_internal_command",
        "baseline_path",
        "baseline_reproduction_command",
        "baseline_stderr_path",
        "baseline_stdout_path",
        "current_internal_command",
        "current_path",
        "current_reproduction_command",
        "current_stderr_path",
        "current_stdout_path",
        "query_result_path",
        "witness_path",
    ] {
        assert!(
            !artifact.contains_key(key),
            "artifact should omit `{key}` when the corresponding file is absent"
        );
    }
}
