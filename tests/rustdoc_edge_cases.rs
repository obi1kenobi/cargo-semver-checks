use assert_cmd::Command;
use std::{path::Path, path::PathBuf, time::SystemTime, time::UNIX_EPOCH};

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "cargo-semver-checks-rustdoc-version-mismatch-{}-{nonce}",
            std::process::id()
        ));
        fs_err::create_dir_all(&path).expect("failed to create test directory");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        if let Err(error) = fs_err::remove_dir_all(&self.0) {
            eprintln!(
                "failed to remove test directory {}: {error}",
                self.0.display()
            );
        }
    }
}

fn cargo_semver_checks() -> Command {
    assert_cmd::cargo::cargo_bin_cmd!("cargo-semver-checks")
}

/// Explain how to resolve incompatible rustdoc output when baseline and current crates select
/// different toolchains.
#[test]
fn different_rustdoc_toolchains_report_actionable_error() {
    let test_dir = TestDirectory::new();
    let baseline = test_dir.path().join("old");
    let current = test_dir.path().join("new");

    for (path, version) in [(&baseline, "0.1.0"), (&current, "0.2.0")] {
        fs_err::create_dir_all(path.join("src"))
            .expect("failed to create fixture source directory");
        fs_err::write(
            path.join("Cargo.toml"),
            format!(
                "[package]\nname = \"rustdoc-version-mismatch-fixture\"\nversion = \"{version}\"\nedition = \"2021\"\n"
            ),
        )
        .expect("failed to write fixture manifest");
        fs_err::write(path.join("src/lib.rs"), "pub fn api() {}\n")
            .expect("failed to write fixture library");
    }

    let baseline_rustdoc = generate_rustdoc(&baseline, &test_dir.path().join("target-baseline"));
    let current_rustdoc = generate_rustdoc(&current, &test_dir.path().join("target-current"));

    // Simulate a baseline from another supported schema without downloading another toolchain.
    let mut baseline_json: serde_json::Value = serde_json::from_str(
        &fs_err::read_to_string(&baseline_rustdoc).expect("failed to read baseline rustdoc JSON"),
    )
    .expect("baseline rustdoc JSON should be valid");
    let current_json: serde_json::Value = serde_json::from_str(
        &fs_err::read_to_string(&current_rustdoc).expect("failed to read current rustdoc JSON"),
    )
    .expect("current rustdoc JSON should be valid");
    let current_format_version = current_json["format_version"]
        .as_u64()
        .expect("current rustdoc JSON should have a numeric format_version");
    let baseline_format_version = if current_format_version == 57 { 60 } else { 57 };
    baseline_json["format_version"] = baseline_format_version.into();
    fs_err::write(
        &baseline_rustdoc,
        serde_json::to_vec(&baseline_json).expect("baseline rustdoc JSON should serialize"),
    )
    .expect("failed to write baseline rustdoc JSON");

    let mut cmd = cargo_semver_checks();
    cmd.current_dir(&current)
        .args(["semver-checks", "check-release", "--baseline-rustdoc"])
        .arg(&baseline_rustdoc)
        .arg("--current-rustdoc")
        .arg(&current_rustdoc)
        .env_remove("RUSTUP_TOOLCHAIN")
        .env("CARGO_TERM_COLOR", "never")
        .env("CARGO_TERM_UNICODE", "false")
        .env("RUST_BACKTRACE", "0")
        .assert()
        .failure()
        .stderr(predicates::str::contains("different Rust toolchains"))
        .stderr(predicates::str::contains(
            "same Rust toolchain for both checkouts",
        ));
}

fn generate_rustdoc(crate_root: &Path, target_dir: &Path) -> PathBuf {
    let manifest = crate_root.join("Cargo.toml");
    let output = std::process::Command::new("cargo")
        .current_dir(crate_root)
        .args(["rustdoc", "--offline", "--manifest-path"])
        .arg(&manifest)
        .arg("--target-dir")
        .arg(target_dir)
        .args([
            "--lib",
            "--",
            "-Z",
            "unstable-options",
            "--output-format",
            "json",
        ])
        .env("RUSTC_BOOTSTRAP", "1")
        .output()
        .expect("failed to start cargo doc");
    assert!(
        output.status.success(),
        "cargo doc failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    target_dir
        .join("doc")
        .join("rustdoc_version_mismatch_fixture.json")
}

/// This test checks whether the tool correctly detects
/// implicit features defined by target-specific dependencies.
/// https://github.com/obi1kenobi/cargo-semver-checks/issues/369
#[test]
fn detects_target_dependencies() {
    let mut cmd = cargo_semver_checks();
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
    let mut cmd = cargo_semver_checks();
    cmd.current_dir("test_crates/lib-target-with-dashes")
        .args(["semver-checks", "check-release", "--baseline-root=."])
        .assert()
        .success();
}

/// Ensure that proc macro targets can be semver-checked.
#[test]
fn proc_macro_target() {
    let mut cmd = cargo_semver_checks();
    cmd.current_dir("test_crates/proc_macro_crate")
        .args(["semver-checks", "check-release", "--baseline-root=."])
        .assert()
        .success();
}

/// Ensure that explicitly selected proc macro crates are checked for API breakage.
/// https://github.com/obi1kenobi/cargo-semver-checks/discussions/1752
#[test]
fn proc_macro_breaking_change() {
    let mut cmd = cargo_semver_checks();
    cmd.current_dir("test_crates/function_like_proc_macro_missing/new")
        .args([
            "semver-checks",
            "check-release",
            "--baseline-root=../old",
            "--package=function_like_proc_macro_missing",
        ])
        .assert()
        .code(100)
        .stdout(predicates::str::contains(
            "failure function_like_proc_macro_missing:",
        ))
        .stdout(predicates::str::contains("macro make_answer in file"));
}

/// Ensure that crates with only a bin target (so, no lib target) produce the correct error message
/// since they have no library API and therefore nothing we can semver-check.
#[test]
fn bin_target_only() {
    let mut cmd = cargo_semver_checks();
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
    let mut cmd = cargo_semver_checks();
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
    let mut cmd = cargo_semver_checks();
    cmd.current_dir("test_crates/rlib_target")
        .args(["semver-checks", "check-release", "--baseline-root=."])
        .assert()
        .success();
}

/// Ensure that crates whose lib target has `crate-type = ["staticlib"]`
/// can be semver-checked correctly.
#[test]
fn staticlib_target() {
    let mut cmd = cargo_semver_checks();
    cmd.current_dir("test_crates/staticlib_target")
        .args(["semver-checks", "check-release", "--baseline-root=."])
        .assert()
        .success();
}

/// Ensure that crates whose lib target has `crate-type = ["dylib"]`
/// can be semver-checked correctly.
#[test]
fn dylib_target() {
    let mut cmd = cargo_semver_checks();
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
    let mut cmd = cargo_semver_checks();
    cmd.current_dir("test_crates/renamed_lib_target")
        .args(["semver-checks", "check-release", "--baseline-root=."])
        .assert()
        .success();
}

/// Ensure dependency warnings do not fail rustdoc generation when the user has
/// `RUSTFLAGS=-Dwarnings` set.
/// https://github.com/obi1kenobi/cargo-semver-checks/issues/589
#[test]
fn dependency_warnings_do_not_fail_rustdoc_generation() {
    let mut cmd = cargo_semver_checks();
    cmd.current_dir("test_crates/dependency_warnings_with_deny/new")
        .args(["semver-checks", "check-release", "--baseline-root=../old"])
        .env("RUSTFLAGS", "-Dwarnings")
        .assert()
        .success();
}

/// Ensure that semver-checking a crate that uses workspace-provided values works fine.
#[test]
fn crate_in_workspace() {
    // Run at workspace level.
    let mut cmd = cargo_semver_checks();
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
    let mut cmd = cargo_semver_checks();
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
    let mut cmd = cargo_semver_checks();
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
    let mut cmd = cargo_semver_checks();
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
    let mut cmd = cargo_semver_checks();
    cmd.current_dir("test_crates/enum_missing/new")
        .args([
            "semver-checks",
            "check-release",
            "--baseline-root=../old",
            "--release-type=minor",
        ])
        .assert()
        .failure();
    let mut cmd = cargo_semver_checks();
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
    let mut cmd = cargo_semver_checks();
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
    let mut cmd = cargo_semver_checks();
    cmd.current_dir("test_crates/type_marked_deprecated/new")
        .args([
            "semver-checks",
            "check-release",
            "--baseline-root=../old",
            "--release-type=patch",
        ])
        .assert()
        .failure();
    let mut cmd = cargo_semver_checks();
    cmd.current_dir("test_crates/type_marked_deprecated/new")
        .args(["semver-checks", "check-release", "--baseline-root=../old"])
        .assert()
        .failure();
}

/// This test ensures that minimum semver version changes for 0.y.z versions
/// are detected correctly without the need for the `--release-type` flag.
/// In this test, the crate has SemVer-minor lints, and the minimum possible
/// SemVer bump is minor (due to the 0.y.z version), so no lints are raised.
#[test]
fn zerover_minor_flag_not_needed() {
    let mut cmd = cargo_semver_checks();
    cmd.current_dir("test_crates/function_marked_deprecated/new")
        .args(["semver-checks", "check-release", "--baseline-root=../old"])
        .assert()
        .success();
}

/// This test ensures that passing `--manifest-path Cargo.toml` doesn't cause a bug.
/// This used to be an issue since computing the parent path of `Cargo.toml` produced
/// an *empty string* instead of the current working directory.
#[test]
fn unprefixed_cargo_toml_manifest_path_refers_to_current_working_directory() {
    let mut cmd = cargo_semver_checks();
    cmd.current_dir("test_crates/crate_in_workspace/")
        .args([
            "semver-checks",
            "check-release",
            "--manifest-path",
            "Cargo.toml",
            "--baseline-root=.",
        ])
        .assert()
        .success();
}

/// This test ensures that passing `--baseline-root ./path/Cargo.toml` doesn't cause a bug.
/// Since `--manifest-path` takes a path, users may assume `--baseline-root` does as well.
/// We strip `Cargo.toml` from the tail of the path if present.
#[test]
fn baseline_root_cargo_toml_path() {
    let mut cmd = cargo_semver_checks();
    cmd.current_dir("test_crates/")
        .args([
            "semver-checks",
            "check-release",
            "--manifest-path",
            "crate_in_workspace/Cargo.toml",
            "--baseline-root=crate_in_workspace/Cargo.toml",
        ])
        .assert()
        .success();
}

/// This test ensures that passing `--baseline-root Cargo.toml` doesn't cause a bug.
/// Since `--manifest-path` takes a path, users may assume `--baseline-root` does as well.
/// We strip `Cargo.toml` from the tail of the path if present.
/// When `Cargo.toml` is the only path component, we implicitly assume `.` is left after stripping.
#[test]
fn baseline_root_bare_cargo_toml_path() {
    let mut cmd = cargo_semver_checks();
    cmd.current_dir("test_crates/crate_in_workspace/")
        .args([
            "semver-checks",
            "check-release",
            "--manifest-path",
            "Cargo.toml",
            "--baseline-root=Cargo.toml",
        ])
        .assert()
        .success();
}
