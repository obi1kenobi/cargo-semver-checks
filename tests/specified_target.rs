use assert_cmd::Command;

fn base() -> Command {
    let mut cmd = Command::cargo_bin("cargo-semver-checks").unwrap();
    cmd.args([
        "semver-checks",
        "check-release",
        "--manifest-path=test_crates/template/new",
        "--baseline-root=test_crates/template/old",
    ]);
    cmd
}

#[test]
fn with_default() {
    base().env_remove("CARGO_BUILD_TARGET").assert().success();
}

#[test]
#[cfg(target_arch = "x86_64")]
fn with_env_var() {
    base()
        .env("CARGO_BUILD_TARGET", "x86_64-unknown-linux-gnu")
        .assert()
        .success();
}

#[test]
#[cfg(target_arch = "x86_64")]
fn with_flag() {
    base()
        .env_remove("CARGO_BUILD_TARGET")
        .arg("--target=x86_64-unknown-linux-gnu")
        .assert()
        .success();
}
