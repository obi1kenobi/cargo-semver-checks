#![cfg(test)]
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
//! `test_outputs/snapshot_tests/{name}.snap` file in your PR.
//!
//! Alternatively, if you can't use `cargo-insta`, review the changed files
//! in the `test_outputs/snapshot_test/ directory by moving `{name}.snap.new` to
//! `{name}.snap` to update the snapshot.  To update all changed tests,
//! run `INSTA_UPDATE=always cargo test --test snapshot_tests`
//!
//! # Adding a new test
//!
//! To add a new test, typically you will want to use the [`integration_test`] helper function
//! with a string invocation of `cargo semver-checks ...` (typically including the `--manifest-path`
//! and `--baseline-root` arguments to specify the location of the current/baseline test crate path,
//! usually in the `test_crates/manifest_tests` directory).  Add a new function marked `#[test]` that
//! calls this function with the prefix (usually the function name) and the arguments.
//!
//! Then run `cargo test --bin cargo_semver_checks snapshot_tests`.  The new test should fail, as
//! there is no snapshot to compare to.  Review the output with `cargo insta review`,
//! and accept it when the captured behavior is correct. (see above if you can't use
//! `cargo-insta`)

use std::{
    cell::RefCell,
    fmt,
    io::{Cursor, Write},
    rc::Rc,
};

use cargo_semver_checks::{Check, GlobalConfig};
use clap::Parser as _;

use crate::Cargo;

/// Helper struct to implement [`Write + 'static`] on a shared buffer.  Single-threaded
/// only, perform write actions then call `StaticWriter::try_into_buffer()` to read.
#[derive(Debug, Default, Clone)]
struct StaticWriter(Rc<RefCell<Cursor<Vec<u8>>>>);

impl StaticWriter {
    #[inline]
    #[must_use]
    fn new() -> Self {
        Self::default()
    }

    /// Returns the buffer if there are no other references to it, otherwise
    /// returns the original [`Rc`]-backed `self`.
    fn try_into_inner(self) -> Result<Vec<u8>, Self> {
        match Rc::try_unwrap(self.0) {
            Ok(b) => Ok(b.into_inner().into_inner()),
            Err(rc) => Err(Self(rc)),
        }
    }
}

impl Write for StaticWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.borrow_mut().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.borrow_mut().flush()
    }
}

/// Struct for printing the result of an invocation of `cargo-semver-checks`
#[derive(Debug)]
struct CommandOutput {
    /// Whether the invocation of `cargo-semver-checks` was successful (i.e., there are no semver-breaking changes),
    /// from [`Report::success`](cargo_semver_checks::Report::success).
    success: bool,
    /// The stderr of the invocation.
    stderr: String,
    /// The stdout of the invocation.
    stdout: String,
}

impl fmt::Display for CommandOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "success: {}", self.success)?;
        writeln!(f, "--- stdout ---\n{}", self.stdout)?;
        writeln!(f, "--- stderr ---\n{}", self.stderr)?;

        Ok(())
    }
}

#[derive(Debug)]
struct CommandResult(anyhow::Result<CommandOutput>);

impl fmt::Display for CommandResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Ok(d) => write!(f, "{d}"),
            Err(e) => writeln!(f, "--- error ---\n{e}"),
        }
    }
}

/// Helper function to assert snapshots of (1) correctly parsed start state
/// and (2) their outputs.  See the module-level documentation for instructions
/// on how to create the tests.
///
/// # Arguments
///
/// - `test_name` is the file prefix for snapshot tests to be saved in as
/// `test_outputs/snapshot_tests/{test_name}-{"args" | "output"}.snap
/// - `invocation` is a list of arguments of the command line invocation,
/// starting with `["cargo", "semver-checks"]`.
#[allow(dead_code)] // TODO
fn assert_integration_test(test_name: &str, invocation: &[&str]) {
    // remove the backtrace environment variable, as this may cause non-
    // reproducable snapshots.
    std::env::remove_var("RUST_BACKTRACE");

    let stdout = StaticWriter::new();
    let stderr = StaticWriter::new();

    let Cargo::SemverChecks(arguments) = Cargo::parse_from(invocation.into_iter());

    let mut config = GlobalConfig::new();

    config.set_stdout(Box::new(stdout.clone()));
    config.set_stderr(Box::new(stderr.clone()));
    config.set_color_choice(false);

    let check = Check::from(arguments.check_release);

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../test_outputs/snapshot_tests");
    let _grd = settings.bind_to_scope();

    insta::assert_ron_snapshot!(format!("{test_name}-input"), check);

    let result = check.check_release(&mut config);

    // drop other references to stdout/err
    drop(config);

    let stdout = stdout
        .try_into_inner()
        .expect("failed to get unique reference to stdout");
    let stderr = stderr
        .try_into_inner()
        .expect("failed to get unique reference to stderr");

    let result = CommandResult(result.map(|report| CommandOutput {
        success: report.success(),
        stdout: String::from_utf8(stdout).expect("failed to convert to UTF-8"),
        stderr: String::from_utf8(stderr).expect("failed to convert to UTF-8"),
    }));

    insta::assert_snapshot!(format!("{test_name}-output"), result);
}

/// [#163](https://github.com/obi1kenobi/cargo-semver-checks/issues/163)
///
/// Running `cargo semver-checks --workspace` on a workspace that has library
/// targets should be an error.
#[test]
fn workspace_no_lib_targets_error() {
    assert_integration_test(
        "workspace_no_lib_targets",
        &[
            "cargo",
            "semver-checks",
            "--manifest-path",
            "test_crates/manifest_tests/no_lib_targets/new",
            "--baseline-root",
            "test_crates/manifest_tests/no_lib_targets/old",
            "--workspace",
        ],
    );
}
