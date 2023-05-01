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
fn simple_no_implicit_features_test() {
    CargoSemverChecks::new(
        "test_crates/features_test/new/",
        "test_crates/features_test/old/Cargo.toml",
    )
    .add_arg("--no-implicit-features")
    .run()
    .success();
}

#[test]
fn simple_default_features_test() {
    CargoSemverChecks::new(
        "test_crates/features_test/new/",
        "test_crates/features_test/old/Cargo.toml",
    )
    .add_arg("--default-features")
    .run()
    .failure();
}
