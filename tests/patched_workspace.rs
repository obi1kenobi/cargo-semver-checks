//! Reproduction for the dropped `[patch.crates-io]` table in per-feature
//! synthetic manifests ([issue #1777](https://github.com/obi1kenobi/cargo-semver-checks/issues/1777)).
//!
//! The fixture workspace (`patch_repro_workspace/`) patches a crates.io
//! dependency with a vendored copy that adds one function; a member crate
//! calls it, and a sibling member enables that crate's feature. The feature
//! heuristic then builds the crate's rustdoc per feature through a synthetic
//! project created with `cargo add --path ... --features ...` — and that
//! synthetic project has no `[patch.crates-io]` table, so the dependency
//! re-resolves to crates.io and the build aborts with `E0432` on the
//! patch-only function.
//!
//! Current behavior (this test): the run fails with the unresolved import.
//! Expected after the fix: the patch table is carried into the synthetic
//! project and the run succeeds. When fixing, flip the assertion below.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Recursively copy `from` to `to` (std-only; the fixture is tiny).
fn copy_dir(from: &Path, to: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dest = to.join(entry.file_name());
        if ty.is_dir() {
            copy_dir(&entry.path(), &dest)?;
        } else {
            std::fs::copy(entry.path(), dest)?;
        }
    }
    Ok(())
}

/// Prepares a git-tagged copy of the fixture in a unique temp directory and
/// returns the path to the workspace root. The caller removes the directory.
fn setup_fixture() -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "cargo-semver-checks-patch-repro-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&root);
    copy_dir(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/patch_repro_workspace"),
        &root,
    )
    .expect("fixture copy");

    let run_git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(&root)
            .env("GIT_AUTHOR_NAME", "test")
            .env("GIT_AUTHOR_EMAIL", "test@example.com")
            .env("GIT_COMMITTER_NAME", "test")
            .env("GIT_COMMITTER_EMAIL", "test@example.com")
            .output()
            .expect("git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    run_git(&["init", "-q"]);
    run_git(&["add", "-A"]);
    run_git(&["commit", "-qm", "fixture"]);
    run_git(&["tag", "v0.1.0"]);
    root
}

#[test]
fn patched_dependency_survives_per_feature_rustdoc_builds() {
    let root = setup_fixture();

    let out = Command::new(assert_cmd::cargo::cargo_bin!("cargo-semver-checks"))
        .arg("semver-checks")
        // A whole-workspace scan: the sibling member's feature-enable pulls
        // `app`'s rustdoc through a per-feature synthetic project.
        .arg("--baseline-rev")
        .arg("v0.1.0")
        .current_dir(&root)
        .env("CARGO_TERM_COLOR", "never")
        .output()
        .expect("semver-checks runs");

    let stderr = String::from_utf8_lossy(&out.stderr);
    // The bug: the synthetic per-feature project drops the patch table, the
    // dependency re-resolves to crates.io, and the build dies on the
    // patch-only function.
    assert!(
        stderr.contains("cannot find function `repro_only`"),
        "expected the dropped-patch build failure, got:\n{stderr}"
    );
    assert!(!out.status.success());
    let _ = std::fs::remove_dir_all(&root);
}
