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
//! To add a new test, typically you will want to use `create_command`
//! and add arguments (especially `--manifest-path` and `--baseline-root`).
//! Then, call [`assert_cmd_snapshot!`] with the command.
//!
//! Then run `cargo test --test snapshot_tests`.  The new test should fail, as
//! there is no snapshot to compare to.  Review the output with `cargo insta review`,
//! and accept it when the captured behavior is correct. (see above if you can't use
//! `cargo-insta`)
use std::process::Command;

#[allow(unused_imports)] // TODO
use insta_cmd::assert_cmd_snapshot;

/// Helper function to create a [`Command`] to run `cargo-semver-checks`
/// for snapshot testing with `assert_cmd_snapshot!`
#[allow(dead_code)] // TODO
fn create_command() -> Command {
    let mut cmd = Command::new(insta_cmd::get_cargo_bin("cargo-semver-checks"));

    cmd.arg("semver-checks");
    // We don't want backtraces in our snapshot, as those may change the
    // output but not affect the behavior of the program.
    cmd.env("RUST_BACKTRACE", "0");

    cmd
}
