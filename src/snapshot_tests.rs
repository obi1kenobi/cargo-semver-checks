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
//! We check **multiple stages** of the `cargo-semver-checks` execution of a given command
//! (currently two: the parsed input [`Check`](cargo_semver_checks::Check) and the output
//! of running [`check_release`](cargo_semver_checks::Check::check_release)).  This means
//! you may have to run `cargo test` and `cargo insta review` multiple times when adding or
//! updating a new test if both the input and output change.
//!
//! Alternatively, if you can't use `cargo-insta`, review the changed files
//! in the `test_outputs/snapshot_test/ directory by moving `{name}.snap.new` to
//! `{name}.snap` to update the snapshot.  To update all changed tests,
//! run `INSTA_UPDATE=always cargo test --bin cargo-semver-checks snapshot_tests`
//!
//! # Adding a new test
//!
//! To add a new test, typically you will want to use the [`assert_integration_test`] helper function
//! with a string invocation of `cargo semver-checks ...` (typically including the `--manifest-path`
//! and `--baseline-root` arguments to specify the location of the current/baseline test crate path:
//! in the `test_crates` directory if the test can use cached generated rustdoc files, or in the
//! `test_crates/manifest_tests` directory if the test relies on `Cargo.toml` manifest files).
//! Add a new function marked `#[test]` that calls [`assert_integration_test`] with
//! the prefix (usually the function name) and the arguments to `cargo semver-checks`
//!
//! Then run `cargo test --bin cargo-semver-checks snapshot_tests`.  The new test should fail, as
//! there is no snapshot to compare to.  Review the output with `cargo insta review`,
//! and accept it when the captured behavior is correct. (see above if you can't use
//! `cargo-insta`)

use std::{
    cell::RefCell,
    fmt,
    io::{Cursor, Write},
    path::{Path, PathBuf},
    rc::Rc,
};

use cargo_semver_checks::{Check, GlobalConfig};
use clap::Parser as _;
use semver::Version;

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
    /// The stderr of the invocation.
    stderr: String,
    /// The stdout of the invocation.
    stdout: String,
}

impl fmt::Display for CommandOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "--- stdout ---\n{}", self.stdout)?;
        writeln!(f, "--- stderr ---\n{}", self.stderr)?;

        Ok(())
    }
}

#[derive(Debug)]
struct CommandResult {
    /// Whether the invocation of `cargo-semver-checks` was successful (i.e., there are no semver-breaking changes),
    /// from [`Report::success`](cargo_semver_checks::Report::success), or an `Err` if `cargo-semver-checks` exited
    /// early with an `Err` variant.
    result: anyhow::Result<bool>,
    /// Captured `stdout` and `stderr` for the command run, regardless of whether it was successful.
    output: CommandOutput,
}

impl fmt::Display for CommandResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.result {
            Ok(success) => writeln!(f, "success: {success}")?,
            Err(e) => writeln!(f, "--- error ---\n{e}")?,
        };

        write!(f, "{}", self.output)
    }
}

/// Helper function to assert snapshots of (1) correctly parsed start state
/// and (2) their outputs.  See the module-level documentation for instructions
/// on how to create the tests.
///
/// # Arguments
///
/// - `test_name` is the file prefix for snapshot tests to be saved in as
///   `test_outputs/snapshot_tests/{test_name}-{"args" | "output"}.snap
/// - `invocation` is a list of arguments of the command line invocation,
///   starting with `["cargo", "semver-checks"]`.
fn assert_integration_test(test_name: &str, invocation: &[&str]) {
    // remove the backtrace environment variable, as this may cause non-
    // reproducible snapshots.
    std::env::remove_var("RUST_BACKTRACE");
    // remove the cargo verbosity variable, which gets passed to `cargo doc`
    // and may create a nonreproducible environment.
    std::env::remove_var("CARGO_TERM_VERBOSE");

    let stdout = StaticWriter::new();
    let stderr = StaticWriter::new();

    let Cargo::SemverChecks(arguments) = Cargo::parse_from(invocation);

    let mut config = GlobalConfig::new();

    config.set_stdout(Box::new(stdout.clone()));
    config.set_stderr(Box::new(stderr.clone()));
    config.set_color_choice(false);
    config.set_log_level(arguments.verbosity.log_level());

    let check = Check::from(arguments.check_release);

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../test_outputs/snapshot_tests");
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
    settings.add_filter("    Blocking waiting for file lock on package cache\n", "");
    // Filter out the current `cargo-semver-checks` version in links to lint references,
    // as this will break across version changes.
    settings.add_filter(
        r"v\d+\.\d+\.\d+(-[\w\.-]+)?/src/lints",
        "[VERSION]/src/lints",
    );

    // The `settings` are applied to the current thread as long as the returned
    // drop guard  `_grd` is alive, so we use a `let` binding to keep it alive
    // for the scope of the function.
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

    let stdout = String::from_utf8(stdout).expect("failed to convert to UTF-8");
    let stderr = String::from_utf8(stderr).expect("failed to convert to UTF-8");

    let result = CommandResult {
        result: result.map(|report| report.success()),
        output: CommandOutput { stderr, stdout },
    };

    insta::assert_snapshot!(format!("{test_name}-output"), result);
}

/// Helper function to get the root of the source code repository, for
/// filtering the path in snapshots.
fn get_root_path() -> PathBuf {
    let canonicalized = Path::new(file!())
        .canonicalize()
        .expect("canonicalization failed");
    // this file is in `$ROOT/src/snapshot_tests.rs`, so the repo root is two `parent`s up.
    let repo_root = canonicalized
        .parent()
        .and_then(Path::parent)
        .expect("getting repo root failed");

    repo_root.to_owned()
}

/// [#163](https://github.com/obi1kenobi/cargo-semver-checks/issues/163)
///
/// Running `cargo semver-checks --workspace` on a workspace that doesn't
/// have any library targets should be an error.
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

/// [#424](https://github.com/obi1kenobi/cargo-semver-checks/issues/424)
///
/// Running `cargo semver-checks --workspace` on a workspace whose members are all
/// `publish = false`.
#[test]
fn workspace_all_publish_false() {
    assert_integration_test(
        "workspace_all_publish_false",
        &[
            "cargo",
            "semver-checks",
            "--manifest-path",
            "test_crates/manifest_tests/workspace_all_publish_false/new",
            "--baseline-root",
            "test_crates/manifest_tests/workspace_all_publish_false/old",
            "--workspace",
        ],
    );
}

/// Running `cargo semver-checks` on a workspace with a `publish = false` member,
/// explicitly including that member with `--package`.  Currently, this executes
/// `cargo semver-checks`.
#[test]
fn workspace_publish_false_explicit() {
    assert_integration_test(
        "workspace_publish_false_explicit",
        &[
            "cargo",
            "semver-checks",
            "--manifest-path",
            "test_crates/manifest_tests/workspace_all_publish_false/new",
            "--baseline-root",
            "test_crates/manifest_tests/workspace_all_publish_false/old",
            "--package",
            "a",
        ],
    )
}

/// Running `cargo semver-checks` on a workspace with a `package = false` member,
/// explicitly including that member with `--package` and also specifying `--workspace`.
/// Currently, `--workspace` overrides `--package` and the `publish = false` member is not
/// semver-checked, and `cargo-semver-checks` silently exits 0.
///
/// Changing this behavior to make it more consistent is tracked in:
/// https://github.com/obi1kenobi/cargo-semver-checks/issues/868
#[test]
fn workspace_publish_false_workspace_flag() {
    assert_integration_test(
        "workspace_publish_false_workspace_flag",
        &[
            "cargo",
            "semver-checks",
            "--manifest-path",
            "test_crates/manifest_tests/workspace_all_publish_false/new",
            "--baseline-root",
            "test_crates/manifest_tests/workspace_all_publish_false/old",
            "--workspace",
            "--package",
            "a",
            // use verbose mode to show what is being skipped
            "--verbose",
        ],
    )
}

/// When a workspace has a crate with a compile error in the baseline version
/// and the user request to semver-check the `--workspace`, which has other workspace
/// members that do not have compile errors.
///
/// Currently, the workspace `semver-checks` all non-error workspace members but returns
/// an error at the end.
#[test]
fn workspace_baseline_compile_error() {
    // HACK: the `cargo doc` error output changed from cargo 1.77 to 1.78, and the snapshot
    // does not work for older versions
    if rustc_version::version().map_or(true, |version| version < Version::new(1, 78, 0)) {
        eprintln!(
            "Skipping this test as `cargo doc` output is different in earlier versions.
            Consider rerunning with cargo >= 1.78"
        );
        return;
    }

    assert_integration_test(
        "workspace_baseline_compile_error",
        &[
            "cargo",
            "semver-checks",
            "--baseline-root",
            "test_crates/manifest_tests/workspace_baseline_compile_error/old",
            "--manifest-path",
            "test_crates/manifest_tests/workspace_baseline_compile_error/new",
            "--workspace",
        ],
    );
}

/// Pin down the behavior when running `cargo-semver-checks` on a project that
/// for some reason contains multiple definitions of the same package name
/// in different workspaces in the same directory.
///
/// The current behavior *is not* necessarily preferable in the long term, and may change.
/// It looks through all `Cargo.toml` files in the directory and accumulates everything they define.
///
/// In the long run, we may want to use something like `cargo locate-project` to determine
/// which workspace we're currently "inside" and only load its manifests.
/// This approach is described here:
/// <https://github.com/obi1kenobi/cargo-semver-checks/issues/462#issuecomment-1569413532>
#[test]
fn multiple_ambiguous_package_name_definitions() {
    assert_integration_test(
        "multiple_ambiguous_package_name_definitions",
        &[
            "cargo",
            "semver-checks",
            "--baseline-root",
            "test_crates/manifest_tests/multiple_ambiguous_package_name_definitions",
            "--manifest-path",
            "test_crates/manifest_tests/multiple_ambiguous_package_name_definitions",
        ],
    );
}

/// Helper function which lists all files in the directory recursively.
///
/// # Arguments
///
/// - `path` is the path from which all [`std::fs::DirEntry`]'s will be expanded from
fn recurse_list_files(path: impl AsRef<Path>) -> Vec<std::fs::DirEntry> {
    let mut buf = vec![];
    let entries = std::fs::read_dir(path).expect("failed to read the requested path");

    for entry in entries {
        let entry = entry.expect("failed to iterate due to intermittent IO errors");
        let meta = entry.metadata().expect("failed to read file metadata");

        if meta.is_dir() {
            let mut subdir = recurse_list_files(entry.path());
            buf.append(&mut subdir);
        }
        if meta.is_file() {
            buf.push(entry);
        }
    }

    buf
}

/// Make sure that no `.snap.new` snapshots exist.
///
/// This testcase exists as [`insta`] behaves as the following
/// if seeing both a `.snap.new` and a `.snap` file:
/// - If the content of both files is equal,
///   it will pass without failure and remove `.snap.new`-file
/// - If not, it will fail and show the diff
///
/// This behavior is problematic as
/// - we might not pick up on a `.snap.new` file being added as the testcase pass in CI and
/// - future contributors would be confused where this change comes from
///
/// Therefore, we are asserting that no such files exist.
#[test]
fn no_new_snapshots() {
    let files = recurse_list_files("test_outputs/");
    let new_snaps = files
        .into_iter()
        .map(|f| f.path())
        .filter(|f| {
            if let Some(name) = f.file_name().unwrap().to_str() {
                return name.ends_with("snap.new");
            }
            false
        })
        .collect::<Vec<_>>();
    assert_eq!(
        new_snaps,
        Vec::<PathBuf>::new(),
        "`.snap.new` files exit, but should not. Did you\n\
        - forget to run `cargo insta review` or\n\
        - forget to move the `.snap.new` file to `.snap` after verifying \
        the content is exactly as expected?"
    );
}
