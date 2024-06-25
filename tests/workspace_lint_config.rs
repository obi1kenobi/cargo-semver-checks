use assert_cmd::Command;
use predicates::boolean::PredicateBooleanExt;

#[test]
fn test_workspace_config() {
    // TODO: we are only checking for required semver version overrides
    // right now because that is all that is currently implemented.
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

    // 1 minor: function_missing, overridden in workspace and not overridden in package
    let function_missing_predicate = predicates::str::is_match("FAIL(.*)minor(.*)function_missing")
        .expect("function_missing regex should be valid");

    // 1 major: module_missing, overridden in workspace (minor) then overridden in package (major)
    let module_missing_predicate = predicates::str::is_match("FAIL(.*)major(.*)module_missing")
        .expect("module_missing regex should be valid");

    assert.stderr(function_missing_predicate.and(module_missing_predicate));
}
