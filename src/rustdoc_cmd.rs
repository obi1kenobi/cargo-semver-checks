use std::io::Write as _;
use std::path::{Path, PathBuf};

use anyhow::Context;
use itertools::Itertools as _;

use crate::{
    rustdoc_gen::{CrateDataForRustdoc, CrateSource, FeaturesGroup},
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
    pub(crate) fn generate_rustdoc(
        &self,
        config: &mut GlobalConfig,
        build_dir: PathBuf,
        crate_source: &CrateSource,
        crate_data: &CrateDataForRustdoc,
    ) -> anyhow::Result<std::path::PathBuf> {
        let crate_name = crate_source.name()?;
        let version = crate_source.version()?;
        let pkg_spec = format!("{crate_name}@{version}");

        // Generate an empty placeholder project with a dependency on the crate
        // whose rustdoc we need. We take this indirect generation path to avoid issues like:
        // https://github.com/obi1kenobi/cargo-semver-checks/issues/167#issuecomment-1382367128
        let placeholder_manifest =
            create_placeholder_rustdoc_manifest(config, crate_source, crate_data)
                .context("failed to create placeholder manifest")?;
        let placeholder_manifest_path =
            save_placeholder_rustdoc_manifest(build_dir.as_path(), placeholder_manifest)
                .context("failed to save placeholder rustdoc manifest")?;
        if matches!(crate_source, CrateSource::ManifestPath { .. }) {
            // We have to run `cargo update` inside the newly-generated project, to ensure
            // all dependencies of the library we're scanning are up-to-date.
            //
            // Otherwise, we risk having a newer version of a dependency in the baseline arm,
            // and an older version of the same dependency in the current arm. If that dependency
            // started providing stronger guarantees in the newer version, such as newly starting
            // to implementing an auto-trait on an existing type, the baseline could contain
            // types that inherit that stronger guarantee whereas the current would not.
            // That would be reported as a breaking change -- incorrectly so.
            //
            // cargo does not guarantee it'll update dependencies to a newer lockfile
            // if using a path dependency. This bit us in this case:
            // https://github.com/obi1kenobi/cargo-semver-checks/issues/167#issuecomment-2324959305
            let stderr = if self.silence {
                std::process::Stdio::piped()
            } else {
                // Print cargo update progress
                std::process::Stdio::inherit()
            };

            let mut cmd = std::process::Command::new("cargo");
            cmd.stdout(std::process::Stdio::null()) // Don't pollute output
                .stderr(stderr)
                .current_dir(build_dir.as_path())
                .arg("update")
                .arg("--manifest-path")
                .arg(&placeholder_manifest_path);

            // Respect our configured color choice.
            cmd.arg(if config.err_color_choice() {
                "--color=always"
            } else {
                "--color=never"
            });

            let output = cmd.output()?;
            if !output.status.success() {
                if self.silence {
                    config.log_error(|config| {
                        let mut stderr = config.stderr();
                        let delimiter = "-----";
                        writeln!(
                            stderr,
                            "error: running 'cargo update' on crate {crate_name} failed with output:"
                        )?;
                        writeln!(
                            stderr,
                            "{delimiter}\n{}\n{delimiter}\n",
                            String::from_utf8_lossy(&output.stderr)
                        )?;
                        writeln!(
                            stderr,
                            "error: failed to update dependencies for crate {crate_name} v{version}"
                        )?;
                        Ok(())
                    })?;
                } else {
                    config.log_error(|config| {
                        writeln!(
                            config.stderr(),
                            "error: running 'cargo update' on crate {crate_name} v{version} failed, see stderr output above"
                        )?;
                        Ok(())
                    })?;
                }
                config.log_error(|config| {
                    let features =
                        crate_source.feature_list_from_config(config, crate_data.feature_config);
                    let mut stderr = config.stderr();
                    writeln!(
                        stderr,
                        "note: this is unlikely to be a bug in cargo-semver-checks,"
                    )?;
                    writeln!(
                        stderr,
                        "      and is probably an issue with the crate's Cargo.toml"
                    )?;
                    writeln!(
                        stderr,
                        "note: the following command can be used to reproduce the compilation error:"
                    )?;
                    let repro_base = produce_repro_workspace(crate_name, crate_source, &features);
                    writeln!(
                        stderr,
                        "{repro_base}cargo update\n"
                    )?;
                    Ok(())
                })?;
                anyhow::bail!(
                    "aborting due to failure to run 'cargo update' for crate {crate_name} v{version}"
                );
            }
        }

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

        // Generating rustdoc JSON for a crate also involves checking that crate's dependencies.
        // The check is done by rustc, not rustdoc, so it's subject to RUSTFLAGS not RUSTDOCFLAGS.
        // We don't want rustc to fail that check if the user has set RUSTFLAGS="-Dwarnings" here.
        // This fixes: https://github.com/obi1kenobi/cargo-semver-checks/issues/589
        let rustflags = match std::env::var("RUSTFLAGS") {
            Ok(mut prior_rustflags) => {
                prior_rustflags.push_str(" --cap-lints=allow");
                std::borrow::Cow::Owned(prior_rustflags)
            }
            Err(_) => std::borrow::Cow::Borrowed("--cap-lints=allow"),
        };

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
                "-Z unstable-options --document-private-items --document-hidden-items --output-format=json --cap-lints=allow",
            )
            .env("RUSTFLAGS", rustflags.as_ref())
            .stdout(std::process::Stdio::null()) // Don't pollute output
            .stderr(stderr)
            .arg("doc")
            .arg("--manifest-path")
            .arg(&placeholder_manifest_path)
            .arg("--target-dir")
            .arg(target_dir)
            .arg("--package")
            .arg(pkg_spec)
            .arg("--lib");
        if let Some(build_target) = crate_data.build_target {
            cmd.arg("--target").arg(build_target);
        }
        if !self.deps {
            cmd.arg("--no-deps");
        }

        // Respect our configured color choice
        cmd.arg(if config.err_color_choice() {
            "--color=always"
        } else {
            "--color=never"
        });

        let output = cmd.output()?;
        if !output.status.success() {
            if self.silence {
                config.log_error(|config| {
                    let mut stderr = config.stderr();
                    let delimiter = "-----";
                    writeln!(
                        stderr,
                        "error: running cargo-doc on crate {crate_name} failed with output:"
                    )?;
                    writeln!(
                        stderr,
                        "{delimiter}\n{}\n{delimiter}\n",
                        String::from_utf8_lossy(&output.stderr)
                    )?;
                    writeln!(
                        stderr,
                        "error: failed to build rustdoc for crate {crate_name} v{version}"
                    )?;
                    Ok(())
                })?;
            } else {
                config.log_error(|config| {
                    writeln!(
                        config.stderr(),
                        "error: running cargo-doc on crate {crate_name} v{version} failed, see stderr output above"
                    )?;
                    Ok(())
                })?;
            }
            config.log_error(|config| {
                let features =
                    crate_source.feature_list_from_config(config, crate_data.feature_config);
                let mut stderr = config.stderr();
                writeln!(
                    stderr,
                    "note: this is usually due to a compilation error in the crate,"
                )?;
                writeln!(
                    stderr,
                    "      and is unlikely to be a bug in cargo-semver-checks"
                )?;
                writeln!(
                    stderr,
                    "note: the following command can be used to reproduce the compilation error:"
                )?;
                let repro_base = produce_repro_workspace(crate_name, crate_source, &features);
                writeln!(stderr, "{repro_base}cargo check\n")?;
                Ok(())
            })?;
            anyhow::bail!(
                "aborting due to failure to build rustdoc for crate {crate_name} v{version}"
            );
        }

        let rustdoc_dir = if let Some(build_target) = crate_data.build_target {
            target_dir.join(build_target).join("doc")
        } else {
            // If not passing an explicit `--target` flag, cargo may still pick a target to use
            // instead of the "host" target, based on its config files and environment variables.

            let build_target = {
                let output = std::process::Command::new("cargo")
                    .env("RUSTC_BOOTSTRAP", "1")
                    .args([
                        "config",
                        "-Zunstable-options",
                        "--color=never",
                        "get",
                        "--format=json-value",
                        "build.target",
                    ])
                    .output()?;
                if output.status.success() {
                    serde_json::from_slice::<Option<String>>(&output.stdout)?
                } else if std::str::from_utf8(&output.stderr)
                    .context("non-utf8 cargo output")?
                    // this is the only way to detect a not set config value currently:
                    //      https://github.com/rust-lang/cargo/issues/13223
                    .contains("config value `build.target` is not set")
                {
                    None
                } else {
                    config.log_error(|config| {
                        let delimiter = "-----";
                        let mut stderr = config.stderr();
                        writeln!(
                            stderr,
                            "error: running cargo-config on crate {crate_name} failed with output:"
                        )?;
                        writeln!(
                            stderr,
                            "{delimiter}\n{}\n{delimiter}\n",
                            String::from_utf8_lossy(&output.stderr)
                        )?;

                        writeln!(stderr, "error: unexpected cargo config output for crate {crate_name} v{version}\n")?;
                        writeln!(stderr, "note: this may be a bug in cargo, or a bug in cargo-semver-checks;")?;
                        writeln!(stderr, "      if unsure, feel free to open a GitHub issue on cargo-semver-checks")?;
                        writeln!(stderr, "note: running the following command on the crate should reproduce the error:")?;
                        writeln!(stderr,
                            "      cargo config -Zunstable-options get --format=json-value build.target\n",
                        )?;
                        Ok(())
                    })?;
                    anyhow::bail!(
                        "aborting due to cargo-config failure on crate {crate_name} v{version}"
                    )
                }
            };

            if let Some(build_target) = build_target {
                target_dir.join(build_target).join("doc")
            } else {
                target_dir.join("doc")
            }
        };

        // There's no great way to figure out whether that crate version has a lib target.
        // We can't easily do it via the index, and we can't reliably do it via metadata.
        // We're reduced to this heuristic:
        // - the crate is not in the metadata (since it isn't a valid dependency -- no lib target),
        // - and if we captured stderr, we saw the telltale error message (else, assume it happened)
        // then it must have been lacking a lib target.
        //
        // In an ideal world, we would ignore crate versions without a lib target while
        // choosing a baseline version, and raise this error sooner. Alas, until the index
        // can give us that data more easily, we can't do that in a reasonable way.
        let observed_stderr_but_lib_msg_not_present = if self.silence {
            let stderr_output = String::from_utf8_lossy(&output.stderr);
            !stderr_output.contains("ignoring invalid dependency ")
                || !stderr_output.contains(" which is missing a lib target")
        } else {
            false
        };
        let subject_crate = metadata
            .packages
            .iter()
            .find(|dep| dep.name == crate_name)
            .ok_or_else(|| {
                if !observed_stderr_but_lib_msg_not_present {
                    anyhow::anyhow!(
                        "crate {crate_name} v{version} has no lib target, nothing to check"
                    )
                } else {
                    panic!(
                        "We declared a dependency on crate `{crate_name}`, but it doesn't exist \
in the metadata and stderr didn't mention it was lacking a lib target. This is probably a bug.",
                    );
                }
            })?;

        // Figure out the name of the JSON file where rustdoc will produce the output we want.
        // The name is:
        // - the name of the library-like target of the crate, not the crate's name
        // - but with all `-` chars replaced with `_` instead.
        // Related: https://github.com/obi1kenobi/cargo-semver-checks/issues/432
        if let Some(lib_target) = subject_crate
            .targets
            .iter()
            .find(|target| super::is_lib_like_checkable_target(target))
        {
            let lib_name = lib_target.name.as_str();
            let rustdoc_json_file_name = lib_name.replace('-', "_");

            let json_path = rustdoc_dir.join(format!("{rustdoc_json_file_name}.json"));
            if json_path.exists() {
                return Ok(json_path);
            } else {
                anyhow::bail!(
                    "could not find expected rustdoc output for `{}`: {}",
                    crate_name,
                    json_path.display()
                );
            }
        }

        anyhow::bail!(
            "aborting since crate {crate_name} v{} has no lib target, so there's nothing to check",
            crate_source.version()?
        )
    }
}

fn produce_repro_workspace(
    crate_name: &str,
    crate_source: &CrateSource,
    features: &[String],
) -> String {
    let selector = match crate_source {
        CrateSource::Registry { version, .. } => format!("{crate_name}@={version}"),
        CrateSource::ManifestPath { manifest } => format!(
            "--path {}",
            manifest
                .path
                .parent()
                .expect("source Cargo.toml had no parent path")
                .to_str()
                .expect("failed to create path string")
        ),
    };
    let feature_list = if features.is_empty() {
        "".to_string()
    } else {
        format!("--features {} ", features.iter().join(","))
    };
    format!(
        "      \
    cargo new --lib example &&
          cd example &&
          echo '[workspace]' >> Cargo.toml &&
          cargo add {selector} --no-default-features {feature_list}&&
          "
    )
}

impl Default for RustdocCommand {
    fn default() -> Self {
        Self::new()
    }
}

/// To get the rustdoc of the project, we first create a placeholder project somewhere
/// with the project as a dependency, and run `cargo rustdoc` on it.
fn create_placeholder_rustdoc_manifest(
    config: &mut GlobalConfig,
    crate_source: &CrateSource,
    crate_data: &CrateDataForRustdoc,
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
                CrateSource::Registry { version, .. } => DependencyDetail {
                    // We need the *exact* version as a dependency, or else cargo will
                    // give us the latest semver-compatible version which is not we want.
                    // Fixes: https://github.com/obi1kenobi/cargo-semver-checks/issues/261
                    version: Some(format!("={version}")),
                    features: crate_source
                        .feature_list_from_config(config, crate_data.feature_config),
                    default_features: matches!(
                        crate_data.feature_config.features_group,
                        FeaturesGroup::Default | FeaturesGroup::Heuristic | FeaturesGroup::All,
                    ),
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
                    features: crate_source
                        .feature_list_from_config(config, crate_data.feature_config),
                    default_features: matches!(
                        crate_data.feature_config.features_group,
                        FeaturesGroup::Default | FeaturesGroup::Heuristic | FeaturesGroup::All,
                    ),
                    ..DependencyDetail::default()
                },
            };
            config.log_verbose(|config| {
                if project_with_features.features.is_empty() {
                    return Ok(());
                }
                writeln!(
                    config.stderr(),
                    "             Features: {}",
                    project_with_features.features.join(","),
                )?;
                Ok(())
            })?;
            let mut deps = DepsSet::new();
            deps.insert(
                crate_source.name()?.to_string(),
                Dependency::Detailed(Box::new(project_with_features)),
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
