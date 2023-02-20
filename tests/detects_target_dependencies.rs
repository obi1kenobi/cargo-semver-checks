use assert_cmd::Command;

#[test]
fn detects_target_dependencies() {
    // This test checks whether the tool correctly detects
    // implicit features defined by target-specific dependencies.
    // https://github.com/obi1kenobi/cargo-semver-checks/issues/369
    let mut cmd = Command::cargo_bin("cargo-semver-checks").unwrap();
    cmd.current_dir(format!("test_crates/target-specific-dependencies"))
        .args([
            "semver-checks",
            "check-release",
            "--baseline-root=.",
        ])
        .assert()
        .success();
}
