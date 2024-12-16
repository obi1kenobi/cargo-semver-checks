//! Integration snapshot tests for testing `cargo-semver-checks` binary behavior.
//!
//! These tests have greater compile time penalties than `src/snapshot_tests.rs`.
//! Prefer those unless your test needs to access behavior in the `main` function of
//! `cargo-semver-checks`.
//!
//! See the module-level doc comment on `src/snapshot_tests.rs` for information on how
//! to create and update snapshot tests.
//!
//! To write an integration test, make a `#[test]` function and call [`assert_integration_test`]
//! to configure and run an integration test on the `cargo-semver-checks` binary.

use std::path::{Path, PathBuf};

use assert_cmd::cargo::CommandCargoExt;
use insta_cmd::Command;

/// Create a snapshot of the output of a `cargo semver-checks` invocation, using [`insta_cmd`].
/// Add arguments and mutate the command using the passed closure (you do not need to include the
/// `semver-checks` subcommand`.  This also lets you modify the [`insta::Settings`], e.g., to add
/// filters.
///
/// The snapshot will be the `test_crates/snapshot_tests/integration_snapshots__{test_name}.snap`
/// file, so make sure to check this file into version control when adding/updating a test.
/// `test_name` will often be the name of the `#[test]` function that calls this function.
fn assert_integration_test(
    test_name: &str,
    modify: impl FnOnce(&mut Command, &mut insta::Settings),
) {
    let mut settings = insta::Settings::clone_current();
    let mut cmd =
        Command::cargo_bin("cargo-semver-checks").expect("failed to get cargo-semver-checks");

    // never use color for more readable snapshots
    cmd.env("CARGO_TERM_COLOR", "never");
    // disable backtrace printing for reproducibility
    cmd.env("RUST_BACKTRACE", "0");

    cmd.arg("semver-checks");
    settings.set_snapshot_path("../test_outputs/");

    modify(&mut cmd, &mut settings);

    settings.bind(|| insta_cmd::assert_cmd_snapshot!(test_name, cmd));
}

/// Snapshots the behavior of the `--bugreport` argument.
#[test]
fn bugreport() {
    assert_integration_test("bugreport", |cmd, settings| {
        cmd.arg("--bugreport");
        // much of this is not reproducible on different machines, so we have
        // to heavily filter it.
        settings.add_filter(
            r"cargo-semver-checks \d+\.\d+\.\d+(-[\w\.-]+)? (\(\w+(-modified)?\))?",
            "cargo-semver-checks [VERSION] ([HASH])",
        );
        settings.add_filter(
            r"#### Operating system\n\n[^\n]+",
            "#### Operating system\n\n[OS]",
        );
        settings.add_filter(
            &regex::escape(executable_path().to_str().expect("non-UTF-8 path")),
            "[EXECUTABLE_PATH]",
        );

        settings.add_filter(
            r"cargo \d+\.\d+\.\d+(-[\w\.-]+)? \(.+\)",
            "cargo [CARGO_VERSION] ([CARGO_VERSION_DETAILS])",
        );

        settings.add_filter(r"Profile: \w+", "Profile: [PROFILE]");
        settings.add_filter(r"Target triple: [\w-]+", "Target triple: [TARGET_TRIPLE]");
        settings.add_filter(r"Family: \w+", "Family: [FAMILY]");
        settings.add_filter(r"OS: \w+", "OS: [OS]");
        settings.add_filter(r"Architecture: \w+", "Architecture: [ARCH]");
        settings.add_filter(r"Pointer width: \d+", "Pointer width: [WIDTH]");
        settings.add_filter(r"Endian: \w+", "Endian: [ENDIAN]");
        settings.add_filter(r"CPU features: [\w,-\.\d]+", "CPU features: [FEATURES]");
        settings.add_filter(r"Host: [\w-]+", "Host: [HOST_TRIPLE]");

        // this should be serialized TOML
        settings.add_filter(
            "Cargo build configuration:\\n\
            --------------------------\\n\
            [\\s\\S]*\\n\
            Please file an issue",
            "Cargo build configuration:\n\
            --------------------------\n\
            [CARGO_BUILD_TOML]\n\
            Please file an issue",
        );

        settings.add_filter(
            r"https://github\.com/obi1kenobi/cargo-semver-checks/issues/new\?\S+",
            "https://github.com/obi1kenobi/cargo-semver-checks/issues/new?[INFO_URLENCODED]",
        );
    });
}

// TODO: this test will break when the `--witness-hints` is stabilized.  It will need to
// be replaced with a different unstable option.  See the module-level doc comment for
// how to update this test.
/// Tests the behavior of unstable options being passed without `-Z unstable-options`.
#[test]
fn unstable_options_without_flag() {
    assert_integration_test("unstable_options_without_flag", |cmd, _| {
        cmd.arg("--witness-hints");
    });
}

/// Snapshots the behavior of `-Z help`.  This snapshot will need to be updated when any
/// unstable options or feature flags are added, removed, or stabilized.  See the module-level
/// doc comment for how to update this test.
#[test]
fn z_help() {
    assert_integration_test("z_help", |cmd, _| {
        cmd.args(["-Z", "help"]);
    })
}

/// Pin down the behavior when running `cargo-semver-checks` on a package that
/// relies on `--cfg` based conditional compilation to enable or disable functionality.
///
/// An example of such a crate in the wild:
/// https://docs.rs/aes/latest/aes/#configuration-flags
///
/// Since `RUSTDOCFLAGS` is set to `--cfg custom`, the crate should be checked with that setting.
/// This should reveal two breaking changes -- one caused by `cfg` in the baseline
/// and one caused by `cfg` in the current.
#[test]
fn cfg_conditional_compilation() {
    assert_integration_test("cfg_conditional_compilation", |cmd, settings| {
        cmd.args([
            "--manifest-path",
            "test_crates/cfg_conditional_compilation/new",
            "--baseline-root",
            "test_crates/cfg_conditional_compilation/old",
        ])
        .env("RUSTDOCFLAGS", "--cfg custom");

        set_snapshot_filters(settings);
    });
}

/// Analogous test to the one above, but when the `--cfg` is not set.
/// In this case, no breakage should be reported.
#[test]
fn cfg_conditional_compilation_without_cfg_set() {
    assert_integration_test(
        "cfg_conditional_compilation_without_cfg_set",
        |cmd, settings| {
            cmd.args([
                "--manifest-path",
                "test_crates/cfg_conditional_compilation/new",
                "--baseline-root",
                "test_crates/cfg_conditional_compilation/old",
            ]);

            set_snapshot_filters(settings);
        },
    );
}

fn set_snapshot_filters(settings: &mut insta::Settings) {
    // Turn dynamic time strings like [  0.123s] into [TIME] for reproducibility.
    settings.add_filter(r"\[\s*[\d\.]+s\]", "[TIME]");
    // Turn total number of checks into [TOTAL] to not fail when new lints are added.
    settings.add_filter(r"\d+ checks", "[TOTAL] checks");
    // Similarly, turn the number of passed checks to also not fail when new lints are added.
    settings.add_filter(r"\d+ pass", "[PASS] pass");
    // Escape the root path (e.g., in lint spans) for deterministic results in different
    // build environments.
    let repo_root = get_root_path();
    settings.add_filter(&regex::escape(&repo_root.to_string_lossy()), "[ROOT]");
    // Remove cargo blocking lines (e.g. from `cargo doc` output) as the amount of blocks
    // is not reproducible.
    settings.add_filter("    Blocking waiting for file lock on [^\n]+\n", "");
    // Filter out the current `cargo-semver-checks` version in links to lint references,
    // as this will break across version changes.
    settings.add_filter(
        r"v\d+\.\d+\.\d+(-[\w\.-]+)?/src/lints",
        "[VERSION]/src/lints",
    );
}

/// Helper function to get the root of the source code repository, for
/// filtering the path in snapshots.
fn get_root_path() -> PathBuf {
    let canonicalized = Path::new(file!())
        .canonicalize()
        .expect("canonicalization failed");
    // this file is in `$ROOT/tests/integration_snapshots.rs`, so the repo root is two `parent`s up.
    let repo_root = canonicalized
        .parent()
        .and_then(Path::parent)
        .expect("getting repo root failed");

    repo_root.to_owned()
}

/// Helper function to get a canonicalized version of the cargo executable bin.
fn executable_path() -> PathBuf {
    Path::new(
        Command::cargo_bin("cargo-semver-checks")
            .expect("expected cargo-semver-checks")
            .get_program(),
    )
    .canonicalize()
    .expect("error canonicalizing")
}
