use assert_cmd::Command;

fn check_paths(current_path: &'static str, baseline_path: &'static str) {
    let mut cmd: Command = assert_cmd::cargo::cargo_bin_cmd!("cargo-semver-checks");
    cmd.args([
        "semver-checks",
        "check-release",
        format!("--manifest-path={current_path}").as_str(),
        format!("--baseline-root={baseline_path}").as_str(),
    ])
    .assert()
    .success();
}

#[test]
fn both_passing_manifest_path_and_directory_works() {
    check_paths("test_crates/template/new/", "test_crates/template/old/");
    check_paths(
        "test_crates/template/new/",
        "test_crates/template/old/Cargo.toml",
    );
    check_paths(
        "test_crates/template/new/Cargo.toml",
        "test_crates/template/old/",
    );
    check_paths(
        "test_crates/template/new/Cargo.toml",
        "test_crates/template/old/Cargo.toml",
    );
}
