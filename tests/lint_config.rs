use assert_cmd::Command;
use predicates::boolean::PredicateBooleanExt;

#[test]
fn test_workspace_config() {
    let mut cmd = Command::cargo_bin("cargo-semver-checks")
        .expect("cargo semver-checks command should exist");

    cmd.current_dir("test_crates/manifest_tests/workspace_overrides")
        .args([
            "semver-checks",
            "--baseline-root",
            "old", // should be the workspace root, not package root
            "--manifest-path",
            "new/pkg/Cargo.toml",
            "-v", // needed to show pass/fail of individual lints
        ]);

    let assert = cmd.assert();

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
    let mut cmd = Command::cargo_bin("cargo-semver-checks")
        .expect("cargo semver-checks command should exist");

    cmd.current_dir("test_crates/manifest_tests/only_warnings")
        .args([
            "semver-checks",
            "--baseline-root",
            "old",
            "--manifest-path",
            "new/",
            "-v", // needed to show pass/fail of individual lints
        ]);

    let assert = cmd.assert();

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
