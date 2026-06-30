//! End-to-end test for the `--report-output` / `--baseline-report` unstable flags.
//!
//! Asserts that:
//! 1. A plain run against `trait_missing` exits non-zero (findings present).
//! 2. Adding `--report-output` exits 0 and writes the JSON.
//! 3. Re-running with `--baseline-report <that JSON>` exits 0 (findings subtracted).

use std::path::PathBuf;
use std::process::Command;

#[test]
fn baseline_report_round_trip() {
    let bin = assert_cmd::cargo::cargo_bin!("cargo-semver-checks");
    let report_path: PathBuf = std::env::temp_dir().join("csc-baseline-report.json");

    let _ = std::fs::remove_file(&report_path);

    let base_args = [
        "semver-checks",
        "--manifest-path",
        "test_crates/trait_missing/new",
        "--baseline-root",
        "test_crates/trait_missing/old",
    ];

    let status = Command::new(bin)
        .args(base_args)
        .status()
        .expect("failed to run cargo-semver-checks");
    assert!(!status.success(), "expected non-zero exit on plain run");

    let status = Command::new(bin)
        .args(base_args)
        .args(["-Z", "unstable-options", "--report-output"])
        .arg(&report_path)
        .status()
        .expect("failed to run cargo-semver-checks");
    assert!(status.success(), "expected exit 0 with --report-output");
    assert!(
        report_path.exists(),
        "expected --report-output to write the JSON"
    );

    let status = Command::new(bin)
        .args(base_args)
        .args(["-Z", "unstable-options", "--baseline-report"])
        .arg(&report_path)
        .status()
        .expect("failed to run cargo-semver-checks");
    assert!(status.success(), "expected exit 0 with --baseline-report");
}
