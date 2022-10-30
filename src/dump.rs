#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RustDocCommand {
    deps: bool,
    silence: bool,
}

impl RustDocCommand {
    pub fn new() -> Self {
        Self {
            deps: false,
            silence: false,
        }
    }

    /// Include dependencies
    ///
    /// Reasons to have this disabled:
    /// - Faster API extraction
    /// - Less likely to hit bugs in rustdoc, like
    ///   - rust-lang/rust#89097
    ///   - rust-lang/rust#83718
    ///
    /// Reasons to have this enabled:
    /// - Check for accidental inclusion of dependencies in your API
    /// - Detect breaking changes from dependencies in your API
    pub fn deps(mut self, yes: bool) -> Self {
        self.deps = yes;
        self
    }

    /// Don't write progress to stderr
    pub fn silence(mut self, yes: bool) -> Self {
        self.silence = yes;
        self
    }

    pub fn dump(
        &self,
        manifest_path: &std::path::Path,
        pkg_spec: Option<&str>,
    ) -> anyhow::Result<std::path::PathBuf> {
        let metadata = cargo_metadata::MetadataCommand::new()
            .manifest_path(manifest_path)
            .no_deps()
            .exec()?;
        let manifest_target_directory = metadata
            .target_directory
            .as_path()
            .as_std_path()
            // HACK: Avoid potential errors when mixing toolchains
            .join(crate::util::SCOPE)
            .join("target");
        let target_dir = manifest_target_directory.as_path();

        let stderr = if self.silence {
            std::process::Stdio::piped()
        } else {
            // Print cargo doc progress
            std::process::Stdio::inherit()
        };

        let mut cmd = std::process::Command::new("cargo");
        cmd.env("RUSTC_BOOTSTRAP", "1")
            .env(
                "RUSTDOCFLAGS",
                "-Z unstable-options --document-hidden-items --output-format=json",
            )
            .stdout(std::process::Stdio::null()) // Don't pollute output
            .stderr(stderr)
            .args(["doc", "--all-features"])
            .arg("--manifest-path")
            .arg(manifest_path)
            .arg("--target-dir")
            .arg(target_dir);
        if let Some(pkg_spec) = pkg_spec {
            cmd.arg("--package").arg(pkg_spec);
        }
        if !self.deps {
            cmd.arg("--no-deps");
        }
        let output = cmd.output()?;
        if !output.status.success() {
            if self.silence {
                anyhow::bail!(
                    "Failed when running cargo-doc on {}: {}",
                    manifest_path.display(),
                    String::from_utf8_lossy(&output.stderr)
                )
            } else {
                anyhow::bail!(
                    "Failed when running cargo-doc on {}. See stderr.",
                    manifest_path.display(),
                )
            }
        }

        let manifest = crate::manifest::Manifest::parse(manifest_path)?;

        let lib_target_name = crate::manifest::get_lib_target_name(&manifest)?;
        let json_path = target_dir.join(format!("doc/{}.json", lib_target_name));
        if json_path.exists() {
            return Ok(json_path);
        }

        let first_bin_target_name = crate::manifest::get_first_bin_target_name(&manifest)?;
        let json_path = target_dir.join(format!("doc/{}.json", first_bin_target_name));
        if !json_path.exists() {
            let crate_name = if let Some(pkg_spec) = pkg_spec {
                pkg_spec
                    .split_once('@')
                    .map(|s| s.0)
                    .unwrap_or(pkg_spec)
                    .to_owned()
            } else {
                crate::manifest::get_package_name(&manifest)?
            };

            anyhow::bail!(
                "Could not find expected rustdoc output for `{}`: {}",
                crate_name,
                json_path.display()
            );
        }

        Ok(json_path)
    }
}

impl Default for RustDocCommand {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::RustDocCommand;

    #[test]
    fn rustdoc_for_lib_crate_without_lib_section() {
        RustDocCommand::default()
            .dump(Path::new("./rustdoc_tests/implicit_lib/Cargo.toml"), None)
            .expect("no errors");
    }

    #[test]
    fn rustdoc_for_lib_crate_with_lib_section() {
        RustDocCommand::default()
            .dump(Path::new("./rustdoc_tests/renamed_lib/Cargo.toml"), None)
            .expect("no errors");
    }

    #[test]
    fn rustdoc_for_bin_crate_without_bin_section() {
        RustDocCommand::default()
            .dump(Path::new("./rustdoc_tests/implicit_bin/Cargo.toml"), None)
            .expect("no errors");
    }

    #[test]
    fn rustdoc_for_bin_crate_with_bin_section() {
        RustDocCommand::default()
            .dump(Path::new("./rustdoc_tests/renamed_bin/Cargo.toml"), None)
            .expect("no errors");
    }
}
