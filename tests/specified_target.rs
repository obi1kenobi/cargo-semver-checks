use assert_cmd::Command;

fn base() -> Command {
    let mut cmd: Command = assert_cmd::cargo::cargo_bin_cmd!("cargo-semver-checks");
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
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn with_env_var() {
    base()
        .env("CARGO_BUILD_TARGET", "x86_64-unknown-linux-gnu")
        .assert()
        .success();
}

#[test]
#[cfg(all(target_os = "linux", target_arch = "riscv64"))]
fn with_env_var() {
    base()
        .env("CARGO_BUILD_TARGET", "riscv64gc-unknown-linux-gnu")
        .assert()
        .success();
}

#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn with_flag() {
    base()
        .env_remove("CARGO_BUILD_TARGET")
        .arg("--target=x86_64-unknown-linux-gnu")
        .assert()
        .success();
}

#[test]
#[cfg(all(target_os = "linux", target_arch = "riscv64"))]
fn with_flag() {
    base()
        .env_remove("CARGO_BUILD_TARGET")
        .arg("--target=riscv64gc-unknown-linux-gnu")
        .assert()
        .success();
}
