use assert_cmd::Command;
use predicates::boolean::PredicateBooleanExt;

fn command_for_crate(crate_name: &'static str) -> Command {
    let mut cmd = Command::cargo_bin("cargo-semver-checks")
        .expect("cargo semver-checks command should exist");

    cmd.current_dir(format!("test_crates/manifest_tests/{crate_name}"))
        .args([
            "semver-checks",
            "--baseline-root",
            "old",
            "--manifest-path",
            "new/",
            "-v", // needed to show pass/fail of individual lints
        ]);
    cmd
}

fn command_for_workspace(workspace: &str, package: &str) -> Command {
    let mut cmd = Command::cargo_bin("cargo-semver-checks")
        .expect("cargo semver-checks command should exist");

    cmd.current_dir(format!("test_crates/manifest_tests/{workspace}"))
        .args([
            "semver-checks",
            "--baseline-root",
            "old",
            "--manifest-path",
            &format!("new/{package}"),
            "-v", // needed to show pass/fail of individual lints
        ]);
    cmd
}

#[test]
fn test_workspace_config() {
    let assert = command_for_workspace("workspace_overrides", "pkg").assert();

    // minor: function_missing, overridden in workspace and not overridden in package
    // deny: overriden as warn in workspace then re-overridden as deny in package
    let function_missing_predicate = predicates::str::is_match("FAIL(.*)minor(.*)function_missing")
        .expect("function_missing regex should be valid");

    // major: module_missing, overridden in workspace (minor) then overridden in package (major)
    // warn: overridden as allow in workspace then re-overridden as warn in package
    let module_missing_predicate = predicates::str::is_match("WARN(.*)major(.*)module_missing")
        .expect("module_missing regex should be valid");

    // since function_missing is deny and minor, we need a new minor bump
    let required_bump = predicates::str::contains("semver requires new minor version");
    // since module_missing is major and warn, it should suggest a new major bump since
    // errors only require a minor bump.
    let suggested_bump = predicates::str::contains("produced warnings suggest new major version");

    assert
        .stderr(function_missing_predicate)
        .stderr(module_missing_predicate)
        .stderr(required_bump)
        .stderr(suggested_bump)
        .failure();
}

#[test]
fn test_only_warnings() {
    let assert = command_for_crate("only_warnings").assert();

    let is_warn = predicates::str::is_match("WARN(.*)major(.*)function_missing")
        .expect("regex should be valid");

    let no_failures = predicates::str::contains("FAIL").not();

    let suggests_major = predicates::str::contains("produced warnings suggest new major version");

    assert
        .stderr(is_warn)
        .stderr(no_failures)
        .stderr(suggests_major)
        .success();
}

#[test]
fn test_allow_skipped() {
    // there is an instance of module_missing in the test crate, but
    // we have set `module_missing = "allow"`, so the lint should
    // not even run, and `cargo-semver-checks` should not require a
    // semver upgrade.
    let assert = command_for_crate("allow").assert();
    assert
        .stderr(predicates::str::contains("module_missing").not())
        .success();
}

/// Helper function to assert whether the `struct_missing` lint has been triggered
/// for testing workspace overrides in the `workspace_key` workspace.
fn test_workspace_key_overrided(package: &str, should_succeed: bool) {
    let assert = command_for_workspace("workspace_key_override", package).assert();
    let struct_missing =
        predicates::str::is_match("FAIL(.*)struct_missing").expect("regex should be valid");

    if should_succeed {
        assert.success().stderr(struct_missing.not());
    } else {
        assert.failure().stderr(struct_missing);
    }
}

/// Tests that workspace overrides are applied when the
/// `package.metadata.cargo-semver-checks.lints.workspace = true`
/// key is set for a package.
#[test]
fn test_workspace_key_metadata_true() {
    test_workspace_key_overrided("metadata_true", true);
}

/// Tests that workspace overrides are applied when the
/// `lints.workspace = true`
/// key is set for a package.
#[test]
fn test_workspace_key_cargo_true() {
    test_workspace_key_overrided("cargo_true", true);
}

/// Tests that workspace overrides are not applied when both the
/// `package.metadata.cargo-semver-checks.lints.workspace = true`
/// and `lints.workspace = true` keys are not set for a package.
#[test]
fn test_workspace_key_both_missing() {
    test_workspace_key_overrided("both_missing", false);
}

/// Tests that config overrides are only read from the new/current
/// manifest and not the old/baseline manifest.  In `old/Cargo.toml`,
/// `struct_missing` is configured to allow, but this should not apply
/// and the lint should still be triggered.
#[test]
fn test_only_read_config_from_new_manifest() {
    let assert = command_for_crate("only_read_new_manifest").assert();
    assert
        .stderr(predicates::str::is_match("FAIL(.*)struct_missing").expect("regex should be valid"))
        .failure();
}
