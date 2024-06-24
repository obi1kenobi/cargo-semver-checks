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
        ]);

    let assert = cmd.assert();

    // 1 minor: function_missing, overridden in workspace and not overridden in package
    // 1 major: module_missing, overridden in workspace (minor) then overridden in package (major)
    //
    let pred =
        predicates::str::contains("semver requires new major version: 1 major and 1 minor check");
    assert.stderr(pred);
}
