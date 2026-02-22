use std::path::{Path, PathBuf};

use assert_cmd::Command;

fn base() -> Command {
    let mut cmd: Command = assert_cmd::cargo::cargo_bin_cmd!("cargo-semver-checks");
    cmd.current_dir(env!("CARGO_MANIFEST_DIR"));
    cmd.args([
        "semver-checks",
        "check-release",
        "--manifest-path=test_crates/template/new",
        "--baseline-root=test_crates/template/old",
    ]);
    cmd
}

fn unique_temp_path(prefix: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "cargo-semver-checks-{prefix}-{}-{}",
        std::process::id(),
        rand::random::<u64>()
    ))
}

fn remove_if_exists(path: &Path) {
    if path.exists() {
        fs_err::remove_dir_all(path).expect("failed to remove temporary test directory");
    }
}

#[test]
fn explicit_target_dir_uses_scoped_internal_work_dir() {
    let target_dir = unique_temp_path("target-dir-explicit");
    remove_if_exists(&target_dir);

    base()
        .arg("--target-dir")
        .arg(&target_dir)
        .assert()
        .success();

    assert!(
        target_dir.join("doc/template.json").exists(),
        "expected rustdoc output at {:?}",
        target_dir.join("doc/template.json")
    );
    assert!(
        target_dir.join("semver-checks").exists(),
        "expected internal work dir at {:?}",
        target_dir.join("semver-checks")
    );

    remove_if_exists(&target_dir);
}

#[test]
fn cli_target_dir_overrides_env_target_dir() {
    let cli_target_dir = unique_temp_path("target-dir-cli");
    let env_target_dir = unique_temp_path("target-dir-env");
    remove_if_exists(&cli_target_dir);
    remove_if_exists(&env_target_dir);

    base()
        .arg("--target-dir")
        .arg(&cli_target_dir)
        .env("CARGO_TARGET_DIR", &env_target_dir)
        .assert()
        .success();

    assert!(
        cli_target_dir.join("semver-checks").exists(),
        "expected CLI target dir to be used at {:?}",
        cli_target_dir.join("semver-checks")
    );
    assert!(
        !env_target_dir.exists(),
        "expected env target dir to be unused at {:?}",
        env_target_dir
    );

    remove_if_exists(&cli_target_dir);
    remove_if_exists(&env_target_dir);
}

#[test]
fn default_target_dir_keeps_current_behavior() {
    base().env_remove("CARGO_TARGET_DIR").assert().success();

    assert!(
        Path::new("test_crates/template/new/target/semver-checks").exists(),
        "expected current crate internal work dir to exist"
    );
    assert!(
        Path::new("test_crates/template/old/target/semver-checks").exists(),
        "expected baseline crate internal work dir to exist"
    );
}

fn write_test_crate(path: &Path, lib_body: &str) {
    fs_err::create_dir_all(path.join("src")).expect("failed to create test crate source directory");
    fs_err::write(
        path.join("Cargo.toml"),
        r#"[package]
name = "target_dir_collision_probe"
version = "0.1.0"
edition = "2021"
"#,
    )
    .expect("failed to write test crate Cargo.toml");
    fs_err::write(path.join("src/lib.rs"), lib_body).expect("failed to write test crate lib.rs");
}

fn run_check(
    manifest_path: &Path,
    baseline_root: &Path,
    target_dir: &Path,
) -> assert_cmd::assert::Assert {
    let mut cmd: Command = assert_cmd::cargo::cargo_bin_cmd!("cargo-semver-checks");
    cmd.current_dir(env!("CARGO_MANIFEST_DIR"));
    cmd.args([
        "semver-checks",
        "check-release",
        "--manifest-path",
        manifest_path
            .to_str()
            .expect("manifest path should be valid UTF-8"),
        "--baseline-root",
        baseline_root
            .to_str()
            .expect("baseline path should be valid UTF-8"),
        "--target-dir",
        target_dir
            .to_str()
            .expect("target dir path should be valid UTF-8"),
    ]);
    cmd.assert()
}

#[test]
fn explicit_target_dir_no_cross_run_false_negative() {
    let root = unique_temp_path("target-dir-collision-false-negative");
    remove_if_exists(&root);
    fs_err::create_dir_all(&root).expect("failed to create temporary test root");

    let breaking_old = root.join("breaking_old");
    let breaking_new = root.join("breaking_new");
    let compatible_old = root.join("compatible_old");
    let compatible_new = root.join("compatible_new");

    write_test_crate(&breaking_old, "pub trait RemovedTrait {}\n");
    write_test_crate(&breaking_new, "");
    write_test_crate(&compatible_old, "");
    write_test_crate(&compatible_new, "");

    let shared_target_dir = root.join("shared-target");

    run_check(&compatible_new, &compatible_old, &shared_target_dir).success();

    run_check(&breaking_new, &breaking_old, &shared_target_dir)
        .failure()
        .stdout(predicates::str::contains("failure trait_missing"));

    remove_if_exists(&root);
}

#[test]
fn explicit_target_dir_no_cross_run_false_positive() {
    let root = unique_temp_path("target-dir-collision-false-positive");
    remove_if_exists(&root);
    fs_err::create_dir_all(&root).expect("failed to create temporary test root");

    let breaking_old = root.join("breaking_old");
    let breaking_new = root.join("breaking_new");
    let compatible_old = root.join("compatible_old");
    let compatible_new = root.join("compatible_new");

    write_test_crate(&breaking_old, "pub trait RemovedTrait {}\n");
    write_test_crate(&breaking_new, "");
    write_test_crate(&compatible_old, "");
    write_test_crate(&compatible_new, "");

    let shared_target_dir = root.join("shared-target");

    run_check(&breaking_new, &breaking_old, &shared_target_dir).failure();

    run_check(&compatible_new, &compatible_old, &shared_target_dir)
        .success()
        .stderr(predicates::str::contains("no semver update required"));

    remove_if_exists(&root);
}
