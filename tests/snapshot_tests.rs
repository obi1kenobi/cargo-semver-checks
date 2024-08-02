//! Snapshot tests of `cargo semver-checks` runs to ensure
//! that we define how we handle edge cases.
//!
//! # Updating test output
//!
//! If you introduce changes into `cargo-semver-checks` that modify its behavior
//! so that these tests fail, the snapshot may need to be updated.  After you've
//! determined that the new behavior is not a regression and the test should
//! be updated, run the following:
//!
//! `$ cargo insta review` (you may need to `cargo install cargo-insta`)
//!
//! If the changes are intended, to update the test, accept the new output
//! in the `cargo insta review` CLI.  Make sure to commit the
//! `tests/snapshots/{name}.snap` file in your PR.
//!
//! Alternatively, if you can't use `cargo-insta`, review the changed files
//! in the `tests/snapshots/ directory by moving `{name}.snap.new` to
//! `{name}.snap` to update the snapshot.  To update all changed tests,
//! run `INSTA_UPDATE=always cargo test --test snapshot_tests`
//!
//! # Adding a new test
//!
//! To add a new test, typically you will want to use `CommandOutput::new` with
//! a `builder` that adds arguments (especially `--manifest-path` and `--baseline-root`).
//! Then, call [`insta::assert_yaml_snapshot!`] with the `output`.
//!
//! Then run `cargo test --test snapshot_tests`.  The new test should fail, as
//! there is no snapshot to compare to.  Review the output with `cargo insta review`,
//! and accept it when the captured behavior is correct. (see above if you can't use
//! `cargo-insta`)
use std::process::Command;

use assert_cmd::cargo::CommandCargoExt;
use serde::Serialize;

/// Structure to serialize the result of running `cargo-semver-checks`
/// to be tested with insta.
///
/// Typical workflow: run `CommandOutput::new` with a `builder` that adds
/// args to the command, use `insta::assert_yaml_snapshot!(&output)`.
#[derive(Debug, Serialize)]
struct CommandOutput {
    exit_code: Option<i32>,
    stderr: String,
    stdout: String,
}

impl CommandOutput {
    /// Creates a new `CommandOutput` instance by running `cargo semver-checks`.
    ///
    /// Use the `builder` parameter to customize the command (e.g., to add
    /// arguments to the invocation of the command)
    #[must_use]
    fn new(builder: impl FnOnce(&mut Command) -> &mut Command) -> Self {
        let mut cmd = Command::cargo_bin("cargo-semver-checks")
            .expect("cargo semver-checks binary not found");

        cmd.arg("semver-checks");

        builder(&mut cmd);

        let output = cmd.output().expect("executing command failed");
        Self {
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        }
    }
}
