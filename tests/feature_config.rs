use assert_cmd::{assert::Assert, Command};

struct CargoSemverChecks {
    cmd: Command,
    args: Vec<String>,
}

impl CargoSemverChecks {
    fn new(current_path: &str, baseline_path: &str) -> Self {
        Self {
            cmd: Command::cargo_bin("cargo-semver-checks").unwrap(),
            args: vec![
                String::from("semver-checks"),
                String::from("check-release"),
                format!("--manifest-path={current_path}"),
                format!("--baseline-root={baseline_path}"),
            ],
        }
    }

    fn add_arg(&mut self, arg: &str) -> &mut Self {
        self.args.push(String::from(arg));
        self
    }

    fn run(&mut self) -> Assert {
        self.cmd.args(&self.args).assert()
    }
}

#[test]
fn simple_only_explicit_feature() {
    CargoSemverChecks::new(
        "test_crates/features_simple/new/",
        "test_crates/features_simple/old/Cargo.toml",
    )
    .add_arg("--only-explicit-features")
    .run()
    .success();
}

#[test]
fn simple_default_features() {
    CargoSemverChecks::new(
        "test_crates/features_simple/new/",
        "test_crates/features_simple/old/Cargo.toml",
    )
    .add_arg("--default-features")
    .run()
    .failure();
}

#[test]
fn simple_heuristic_features() {
    CargoSemverChecks::new(
        "test_crates/features_simple/new/",
        "test_crates/features_simple/old/Cargo.toml",
    )
    // make sure 'foo' is added to current
    .add_arg("--baseline-features")
    .add_arg("foo")
    .run()
    .success();
}

#[test]
fn simple_all_features() {
    CargoSemverChecks::new(
        "test_crates/features_simple/new/",
        "test_crates/features_simple/old/Cargo.toml",
    )
    .add_arg("--all-features")
    .run()
    .failure();
}

#[test]
fn function_moved_only_explicit_features() {
    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--only-explicit-features")
    .run()
    .success();

    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--only-explicit-features")
    .add_arg("--baseline-features")
    .add_arg("C")
    .run()
    .success();

    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--only-explicit-features")
    .add_arg("--baseline-features")
    .add_arg("A")
    .add_arg("--current-features")
    .add_arg("B")
    .run()
    .success();

    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--only-explicit-features")
    .add_arg("--features")
    .add_arg("B")
    .run()
    .failure();

    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--only-explicit-features")
    .add_arg("--features")
    .add_arg("A")
    .add_arg("--features")
    .add_arg("B")
    .add_arg("--features")
    .add_arg("C")
    .run()
    .success();
}

#[test]
fn function_moved_default_features() {
    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--default-features")
    .run()
    .failure();

    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--default-features")
    .add_arg("--current-features")
    .add_arg("B")
    .run()
    .success();

    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--default-features")
    .add_arg("--features")
    .add_arg("B")
    .run()
    .failure();

    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--default-features")
    .add_arg("--features")
    .add_arg("B")
    .add_arg("--current-features")
    .add_arg("C")
    .run()
    .success();
}

#[test]
fn function_moved_heuristic_features() {
    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .run()
    .success();
}

#[test]
fn function_moved_all_features() {
    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--all-features")
    .run()
    .success();
}

#[test]
fn default_features_when_default_undefined() {
    CargoSemverChecks::new(
        "test_crates/features_no_default/new/",
        "test_crates/features_no_default/old/Cargo.toml",
    )
    .add_arg("--default-features")
    .run()
    .success();

    CargoSemverChecks::new(
        "test_crates/features_no_default/new/",
        "test_crates/features_no_default/old/Cargo.toml",
    )
    .add_arg("--default-features")
    .add_arg("--features")
    .add_arg("A")
    .run()
    .success();

    CargoSemverChecks::new(
        "test_crates/features_no_default/new/",
        "test_crates/features_no_default/old/Cargo.toml",
    )
    .add_arg("--default-features")
    .add_arg("--baseline-features")
    .add_arg("A")
    .run()
    .success();

    CargoSemverChecks::new(
        "test_crates/features_no_default/new/",
        "test_crates/features_no_default/old/Cargo.toml",
    )
    .add_arg("--default-features")
    .add_arg("--current-features")
    .add_arg("B")
    .run()
    .success();

    CargoSemverChecks::new(
        "test_crates/features_no_default/new/",
        "test_crates/features_no_default/old/Cargo.toml",
    )
    .add_arg("--default-features")
    .add_arg("--features")
    .add_arg("B")
    .run()
    .failure();
}

#[test]
fn feature_does_not_exist() {
    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--features")
    .add_arg("new_feature")
    .run()
    .success();

    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--features")
    .add_arg("feature_to_be_removed")
    .run()
    .failure();
}
