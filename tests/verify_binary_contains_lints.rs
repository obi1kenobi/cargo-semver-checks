use assert_cmd::Command;

#[test]
fn verify_binary_contains_lints() {
    let assert_on_crate_pair = |crate_pair: &str| {
        let mut cmd = Command::cargo_bin("cargo-semver-checks").unwrap();
        cmd.current_dir(format!("test_crates/{crate_pair}/new"))
            .args(["semver-checks", "check-release", "--baseline-root=../old/"])
            .assert()
    };

    // The `template/new` and `template/old` are identical crates, so running cargo-semver-checks on
    // them shouldn't report any issues.
    assert_on_crate_pair("template").success();

    // Those test crate pairs should trigger an error (because of a lint with the same name),
    // so they should return a non-zero exit code.
    // Only a few (arbitrarily) lints are being checked to speed up the testing process (the full
    // list of lints is tested in `src/query.rs`).
    for crate_pair in [
        "enum_missing",
        "function_const_removed",
        "function_unsafe_added",
    ] {
        assert_on_crate_pair(crate_pair).failure();
    }
}
