use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::{
    rustdoc_gen::{CrateDataForRustdoc, CrateSource},
    GlobalConfig,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RustdocCommand {
    deps: bool,
    silence: bool,
}

impl RustdocCommand {
    pub(crate) fn new() -> Self {
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
    pub(crate) fn deps(mut self, yes: bool) -> Self {
        self.deps = yes;
        self
    }

    /// Don't write progress to stderr
    pub(crate) fn silence(mut self, yes: bool) -> Self {
        self.silence = yes;
        self
    }

    /// Produce a rustdoc JSON file for the specified crate and source.
    ///
    ///
    pub(crate) fn generate_rustdoc(
        &self,
        config: &mut GlobalConfig,
        build_dir: PathBuf,
        crate_source: &CrateSource,
        crate_data: &CrateDataForRustdoc,
    ) -> anyhow::Result<std::path::PathBuf> {
        // Generate an empty placeholder project with a dependency on the crate
        // whose rustdoc we need. We take this indirect generation path to avoid issues like:
        // https://github.com/obi1kenobi/cargo-semver-checks/issues/167#issuecomment-1382367128
        let placeholder_manifest = create_placeholder_rustdoc_manifest(crate_source, crate_data)
            .context("failed to create placeholder manifest")?;
        let placeholder_manifest_path =
            save_placeholder_rustdoc_manifest(build_dir.as_path(), placeholder_manifest)
                .context("failed to save placeholder rustdoc manifest")?;

        let metadata = cargo_metadata::MetadataCommand::new()
            .manifest_path(&placeholder_manifest_path)
            .exec()?;
        let placeholder_target_directory = metadata
            .target_directory
            .as_path()
            .as_std_path()
            // HACK: Avoid potential errors when mixing toolchains
            .join(crate::util::SCOPE)
            .join("target");
        let target_dir = placeholder_target_directory.as_path();

        let stderr = if self.silence {
            std::process::Stdio::piped()
        } else {
            // Print cargo doc progress
            std::process::Stdio::inherit()
        };

        let crate_name = crate_source.name()?;
        let version = crate_source.version()?;
        let pkg_spec = format!("{crate_name}@{version}");

        // Run the rustdoc generation command on the placeholder crate,
        // specifically requesting the rustdoc of *only* the crate specified in `pkg_spec`.
        //
        // N.B.: Passing `--all-features` here has no effect, since that only applies to
        //       the top-level project i.e. the placeholder, which has no features.
        //       To generate rustdoc for our intended crate with features enabled,
        //       those features must be enabled on the dependency in the `Cargo.toml`
        //       of the placeholder project.
        let mut cmd = std::process::Command::new("cargo");
        cmd.env("RUSTC_BOOTSTRAP", "1")
            .env(
                "RUSTDOCFLAGS",
                "-Z unstable-options --document-private-items --document-hidden-items --output-format=json --cap-lints allow",
            )
            .stdout(std::process::Stdio::null()) // Don't pollute output
            .stderr(stderr)
            .arg("doc")
            .arg("--manifest-path")
            .arg(&placeholder_manifest_path)
            .arg("--target-dir")
            .arg(target_dir)
            .arg("--package")
            .arg(pkg_spec);
        if !self.deps {
            cmd.arg("--no-deps");
        }
        if config.is_stderr_tty() {
            cmd.arg("--color=always");
        }

        let output = cmd.output()?;
        if !output.status.success() {
            if self.silence {
                anyhow::bail!(
                    "Failed when running cargo-doc on {}:\n{}",
                    placeholder_manifest_path.display(),
                    String::from_utf8_lossy(&output.stderr)
                )
            } else {
                anyhow::bail!(
                    "Failed when running cargo-doc on {}. See stderr.",
                    placeholder_manifest_path.display(),
                )
            }
        }

        // Get the name of the library target of the crate whose rustdoc we want.
        // Rustdoc generates the JSON file with the name of the *lib target* of the crate,
        // not the name of the crate itself.
        // Related: https://github.com/obi1kenobi/cargo-semver-checks/issues/432
        let lib_name = metadata
            .packages
            .iter()
            .find(|dep| dep.name == crate_name)
            .expect("we declared a dependency on a crate that doesn't exist in the metadata")
            .targets
            .iter()
            .find(|target| target.is_lib())
            .ok_or_else(|| {
                anyhow::anyhow!("Crate {crate_name} does not seem to have a lib target")
            })?
            .name
            .as_str();
        let json_path = target_dir.join(format!("doc/{lib_name}.json"));
        if json_path.exists() {
            Ok(json_path)
        } else {
            anyhow::bail!(
                "Could not find expected rustdoc output for `{}`: {}",
                crate_name,
                json_path.display()
            );
        }
    }
}

impl Default for RustdocCommand {
    fn default() -> Self {
        Self::new()
    }
}

/// To get the rustdoc of the project, we first create a placeholder project somewhere
/// with the project as a dependency, and run `cargo rustdoc` on it.
fn create_placeholder_rustdoc_manifest(
    crate_source: &CrateSource,
    _crate_data: &CrateDataForRustdoc, // TODO: use this to select crate features to enable
) -> anyhow::Result<cargo_toml::Manifest<()>> {
    use cargo_toml::*;

    Ok(Manifest::<()> {
        package: {
            let mut package = Package::new("rustdoc", "0.0.0");
            package.publish = Inheritable::Set(Publish::Flag(false));
            Some(package)
        },
        workspace: Some(Workspace::<()>::default()),
        lib: {
            let product = Product {
                path: Some("lib.rs".to_string()),
                ..Product::default()
            };
            Some(product)
        },
        dependencies: {
            let project_with_features: DependencyDetail = match crate_source {
                CrateSource::Registry { crate_ } => DependencyDetail {
                    // We need the *exact* version as a dependency, or else cargo will
                    // give us the latest semver-compatible version which is not we want.
                    // Fixes: https://github.com/obi1kenobi/cargo-semver-checks/issues/261
                    version: Some(format!("={}", crate_.version())),
                    features: crate_source.all_features(),
                    ..DependencyDetail::default()
                },
                CrateSource::ManifestPath { manifest } => DependencyDetail {
                    path: Some({
                        let dir_path =
                            crate::manifest::get_project_dir_from_manifest_path(&manifest.path)?;
                        // The manifest will be saved in some other directory,
                        // so for convenience, we're using absolute paths.
                        dir_path
                            .canonicalize()
                            .context("failed to canonicalize manifest path")?
                            .to_str()
                            .context("manifest path is not valid UTF-8")?
                            .to_string()
                    }),
                    features: crate_source.all_features(),
                    ..DependencyDetail::default()
                },
            };
            let mut deps = DepsSet::new();
            deps.insert(
                crate_source.name()?.to_string(),
                Dependency::Detailed(project_with_features),
            );
            deps
        },
        ..Default::default()
    })
}

fn save_placeholder_rustdoc_manifest(
    placeholder_build_dir: &Path,
    placeholder_manifest: cargo_toml::Manifest<()>,
) -> anyhow::Result<PathBuf> {
    std::fs::create_dir_all(placeholder_build_dir).context("failed to create build dir")?;
    let placeholder_manifest_path = placeholder_build_dir.join("Cargo.toml");

    // Possibly fixes https://github.com/libp2p/rust-libp2p/pull/2647#issuecomment-1280221217
    let _: std::io::Result<()> = std::fs::remove_file(placeholder_build_dir.join("Cargo.lock"));

    std::fs::write(
        &placeholder_manifest_path,
        toml::to_string(&placeholder_manifest)?,
    )
    .context("failed to write placeholder manifest")?;
    std::fs::write(placeholder_build_dir.join("lib.rs"), "")
        .context("failed to create empty lib.rs")?;
    Ok(placeholder_manifest_path)
}
