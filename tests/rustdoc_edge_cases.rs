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

/// Ensure that proc macro crates without a lib target produce the correct error message
/// since they have no library API and therefore nothing we can semver-check.
#[test]
fn proc_macro_target() {
    let mut cmd = Command::cargo_bin("cargo-semver-checks").unwrap();
    cmd.current_dir("test_crates/proc_macro_crate")
        .args(["semver-checks", "check-release", "--baseline-root=."])
        .env_remove("RUST_BACKTRACE")
        .assert()
        .stderr(
            "\
error: no crates with library targets selected, nothing to semver-check
note: only library targets contain an API surface that can be checked for semver
note: skipped the following crates since they have no library target: proc_macro_crate\n",
        )
        .failure();
}

/// Ensure that crates with only a bin target (so, no lib target) produce the correct error message
/// since they have no library API and therefore nothing we can semver-check.
#[test]
fn bin_target_only() {
    let mut cmd = Command::cargo_bin("cargo-semver-checks").unwrap();
    cmd.current_dir("test_crates/bin_target_only")
        .args(["semver-checks", "check-release", "--baseline-root=."])
        .env_remove("RUST_BACKTRACE")
        .assert()
        .stderr(
            "\
error: no crates with library targets selected, nothing to semver-check
note: only library targets contain an API surface that can be checked for semver
note: skipped the following crates since they have no library target: bin_target_only\n",
        )
        .failure();
}

/// Ensure that crates whose library target is `doc = false` can still be semver-checked.
#[test]
fn doc_false_lib_target() {
    let mut cmd = Command::cargo_bin("cargo-semver-checks").unwrap();
    cmd.current_dir("test_crates/doc_false_lib_target")
        .args(["semver-checks", "check-release", "--baseline-root=."])
        .env_remove("RUST_BACKTRACE")
        .assert()
        .success();
}

/// Ensure that crates whose lib target has `crate-type = ["rlib"]`
/// can be semver-checked correctly.
#[test]
fn rlib_target() {
    let mut cmd = Command::cargo_bin("cargo-semver-checks").unwrap();
    cmd.current_dir("test_crates/rlib_target")
        .args(["semver-checks", "check-release", "--baseline-root=."])
        .assert()
        .success();
}

/// Ensure that crates whose lib target has `crate-type = ["staticlib"]`
/// can be semver-checked correctly.
#[test]
fn staticlib_target() {
    let mut cmd = Command::cargo_bin("cargo-semver-checks").unwrap();
    cmd.current_dir("test_crates/staticlib_target")
        .args(["semver-checks", "check-release", "--baseline-root=."])
        .assert()
        .success();
}

/// Ensure that crates whose lib target has `crate-type = ["dylib"]`
/// can be semver-checked correctly.
#[test]
fn dylib_target() {
    let mut cmd = Command::cargo_bin("cargo-semver-checks").unwrap();
    cmd.current_dir("test_crates/dylib_target")
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
            "lib_crate",
            "--baseline-root=.",
        ])
        .assert()
        .success();

    // Run at workspace level then point out a crate without a lib target.
    // This should produce an error and a non-zero exit code.
    let mut cmd = Command::cargo_bin("cargo-semver-checks").unwrap();
    cmd.current_dir("test_crates/crate_in_workspace")
        .args([
            "semver-checks",
            "check-release",
            "--manifest-path=./Cargo.toml",
            "-p",
            "non_lib_crate",
            "--baseline-root=.",
        ])
        .env_remove("RUST_BACKTRACE")
        .assert()
        .stderr(
            "\
error: no crates with library targets selected, nothing to semver-check
note: only library targets contain an API surface that can be checked for semver
note: skipped the following crates since they have no library target: non_lib_crate\n",
        )
        .failure();
}

/// This test ensures that the `--release-type` flag works correctly,
/// overriding autodetected version changes between the rustdoc JSON files.
/// https://github.com/obi1kenobi/cargo-semver-checks/issues/438
#[test]
fn release_type_flag_major() {
    // This run checks a crate with a major breaking change that doesn't bump the version.
    // It should pass without errors because of `--release-type=major`.
    let mut cmd = Command::cargo_bin("cargo-semver-checks").unwrap();
    cmd.current_dir("test_crates/enum_missing/new")
        .args([
            "semver-checks",
            "check-release",
            "--baseline-root=../old",
            "--release-type=major",
        ])
        .assert()
        .success();

    // Running the same command with `--release-type=minor` or without that flag fails both times.
    let mut cmd = Command::cargo_bin("cargo-semver-checks").unwrap();
    cmd.current_dir("test_crates/enum_missing/new")
        .args([
            "semver-checks",
            "check-release",
            "--baseline-root=../old",
            "--release-type=minor",
        ])
        .assert()
        .failure();
    let mut cmd = Command::cargo_bin("cargo-semver-checks").unwrap();
    cmd.current_dir("test_crates/enum_missing/new")
        .args(["semver-checks", "check-release", "--baseline-root=../old"])
        .assert()
        .failure();
}

/// This test ensures that the `--release-type` flag works correctly,
/// overriding autodetected version changes between the rustdoc JSON files.
/// https://github.com/obi1kenobi/cargo-semver-checks/issues/438
#[test]
fn release_type_flag_minor() {
    // This run checks a crate with deprecations (semver-minor) that doesn't bump the version.
    // It should pass without errors because of `--release-type=minor`.
    let mut cmd = Command::cargo_bin("cargo-semver-checks").unwrap();
    cmd.current_dir("test_crates/type_marked_deprecated/new")
        .args([
            "semver-checks",
            "check-release",
            "--baseline-root=../old",
            "--release-type=minor",
        ])
        .assert()
        .success();

    // Running the same command with `--release-type=patch` or without that flag fails both times.
    let mut cmd = Command::cargo_bin("cargo-semver-checks").unwrap();
    cmd.current_dir("test_crates/type_marked_deprecated/new")
        .args([
            "semver-checks",
            "check-release",
            "--baseline-root=../old",
            "--release-type=patch",
        ])
        .assert()
        .failure();

    // Running the same command without the flag should pass.
    // This is because the detected minimum version change is semver-minor,
    let mut cmd = Command::cargo_bin("cargo-semver-checks").unwrap();
    cmd.current_dir("test_crates/type_marked_deprecated/new")
        .args(["semver-checks", "check-release", "--baseline-root=../old"])
        .assert()
        .success();
}
