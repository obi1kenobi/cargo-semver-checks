//! Tests the --color flag and CARGO_TERM_COLOR env var, and their interactions
//!
//! The --color flag should take precedence over the environment variable
use assert_cmd::Command;
use predicates::boolean::PredicateBooleanExt;

/// helper function to run `check-release` on a given directory in `test_crates/` with the given color arg/env var
/// - `color_arg` is the argument to `--color=<choice>`, if set
/// - `color_env_var` sets the `CARGO_TERM_COLOR=choice`, if set
///
/// Panics with a failed assertion if there are colors (i.e., ANSI color codes) in the output and `assert_colors` is false, or vice versa
fn run_on_crate_diff(
    test_crate_name: &'static str,
    color_arg: Option<&'static str>,
    color_env_var: Option<&'static str>,
    assert_colors: bool,
) {
    let mut cmd = Command::cargo_bin("cargo-semver-checks").unwrap();

    cmd.current_dir(format!("test_crates/{}", test_crate_name))
        .args([
            "semver-checks",
            "check-release",
            "--manifest-path=new/Cargo.toml",
            "--baseline-root=old/",
        ]);

    if let Some(color_arg) = color_arg {
        cmd.arg("--color").arg(color_arg);
    }

    if let Some(color_env) = color_env_var {
        cmd.env("CARGO_TERM_COLOR", color_env);
    }

    let pred = predicates::str::contains("\u{1b}[");

    let assert = cmd.assert();

    if assert_colors {
        assert.stderr(pred);
    } else {
        assert.stderr(pred.not());
    }
}

/// test for colors when using the --color=always flag
#[test]
fn always_flag() {
    // test on a crate diff returning no errors
    run_on_crate_diff("template", Some("always"), None, true);
    // test on a crate diff that will return errors
    run_on_crate_diff("enum_missing", Some("always"), None, true);
}

/// test for no colors when using the --color=never flag
#[test]
fn never_flag() {
    // test on a crate diff returning no errors
    run_on_crate_diff("template", Some("never"), None, false);
    // test on a crate diff that will return errors
    run_on_crate_diff("enum_missing", Some("never"), None, false);
}

/// test for colors when using the CARGO_TERM_COLOR=always env var
#[test]
fn always_var() {
    // test on a crate diff returning no errors
    run_on_crate_diff("template", None, Some("always"), true);
    // test on a crate diff that will return errors
    run_on_crate_diff("enum_missing", None, Some("always"), true);
}

/// test for colors when using the CARGO_TERM_COLOR=never env var
#[test]
fn never_var() {
    // test on a crate diff returning no errors
    run_on_crate_diff("template", None, Some("never"), false);
    // test on a crate diff that will return errors
    run_on_crate_diff("enum_missing", None, Some("never"), false);
}

/// test for the behavior of the --color flag overriding the `CARGO_TERM_COLOR` env var when they conflict
#[test]
fn cli_flag_overrides_env_var() {
    // test when --color=always and CARGO_TERM_COLOR=never
    // test on a crate diff returning no errors
    run_on_crate_diff("template", Some("always"), Some("never"), true);
    // test on a crate diff that will return errors
    run_on_crate_diff("enum_missing", Some("always"), Some("never"), true);

    // test when --color=never and CARGO_TERM_COLOR=always
    // test on a crate diff returning no errors
    run_on_crate_diff("template", Some("never"), Some("always"), false);
    // test on a crate diff that will return errors
    run_on_crate_diff("enum_missing", Some("never"), Some("always"), false);
}
