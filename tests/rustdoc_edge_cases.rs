use assert_cmd::Command;

/// This test checks whether the tool correctly detects
/// implicit features defined by target-specific dependencies.
/// https://github.com/obi1kenobi/cargo-semver-checks/issues/369
#[test]
fn detects_target_dependencies() {
    let mut cmd = Command::cargo_bin("cargo-semver-checks").unwrap();
    cmd.current_dir("test_crates/target_specific_dependencies")
        .args(["semver-checks", "check-release", "--baseline-root=."])
        .assert()
        .success();
}

/// Ensure that crate and lib target names with dashes work properly.
/// Rustdoc uses the lib target name with dashes replaced with underscores as the JSON file name.
/// https://github.com/obi1kenobi/cargo-semver-checks/issues/432
#[test]
fn lib_target_with_dashes() {
    let mut cmd = Command::cargo_bin("cargo-semver-checks").unwrap();
    cmd.current_dir("test_crates/lib-target-with-dashes")
        .args(["semver-checks", "check-release", "--baseline-root=."])
        .assert()
        .success();
}

/// Ensure that crates whose lib targets have a different name can be semver-checked correctly.
/// Rustdoc uses the lib target name with dashes replaced with underscores as the JSON file name.
/// https://github.com/obi1kenobi/cargo-semver-checks/issues/432
#[test]
fn renamed_lib_target() {
    let mut cmd = Command::cargo_bin("cargo-semver-checks").unwrap();
    cmd.current_dir("test_crates/renamed_lib_target")
        .args(["semver-checks", "check-release", "--baseline-root=."])
        .assert()
        .success();
}

/// Ensure that semver-checking a crate that uses workspace-provided values works fine.
#[test]
fn crate_in_workspace() {
    // Run at workspace level.
    let mut cmd = Command::cargo_bin("cargo-semver-checks").unwrap();
    cmd.current_dir("test_crates/crate_in_workspace")
        .args([
            "semver-checks",
            "check-release",
            "--manifest-path=./Cargo.toml",
            "--baseline-root=.",
        ])
        .assert()
        .success();

    // Run at workspace level but point out the crate explicitly.
    let mut cmd = Command::cargo_bin("cargo-semver-checks").unwrap();
    cmd.current_dir("test_crates/crate_in_workspace")
        .args([
            "semver-checks",
            "check-release",
            "--manifest-path=./Cargo.toml",
            "-p",
            "crate1",
            "--baseline-root=.",
        ])
        .assert()
        .success();
}
