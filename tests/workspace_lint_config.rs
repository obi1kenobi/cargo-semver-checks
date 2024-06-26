use assert_cmd::Command;

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

    // minor: function_missing, overridden in workspace and not overridden in package
    // deny: overriden as warn in workspace then re-overridden as deny in package
    let function_missing_predicate =
        predicates::str::is_match("FAIL(.*)minor(.*)deny(.*)function_missing")
            .expect("function_missing regex should be valid");

    // major: module_missing, overridden in workspace (minor) then overridden in package (major)
    // warn: overridden as allow in workspace then re-overridden as warn in package
    let module_missing_predicate =
        predicates::str::is_match("FAIL(.*)major(.*)warn(.*)module_missing")
            .expect("module_missing regex should be valid");

    // those two lints should be the only ones that fail
    let only_two_failures =
        predicates::function::function(|x: &str| x.matches("FAIL").count() == 2);

    // since function_missing is deny and minor, we need a new minor bump
    let required_bump = predicates::str::contains("semver requires new minor version");
    // since module_missing is major and warn, it should suggest a new major bump since
    // errors only require a minor bump.
    let suggested_bump = predicates::str::contains("warning checks suggest new major version");

    assert
        .stderr(function_missing_predicate)
        .stderr(module_missing_predicate)
        .stderr(only_two_failures)
        .stderr(required_bump)
        .stderr(suggested_bump);
}
