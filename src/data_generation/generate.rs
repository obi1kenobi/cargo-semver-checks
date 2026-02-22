use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use anyhow::Context as _;
use itertools::Itertools;

use crate::data_generation::request::RequestKind;

use super::error::{IntoTerminalResult as _, TerminalError};
use super::progress::CallbackHandler;
use super::request::CrateDataRequest;

#[derive(Debug, Clone, Copy)]
pub(crate) struct GenerationSettings {
    /// Whether to allow `cargo` invocations to print directly to our stderr (`true`)
    /// or pipe the output to a buffer available after-the-fact (`false`).
    pub(crate) pass_through_stderr: bool,

    /// On `true`, pass `--color=always` to `cargo` invocations. On `false`, pass `--color=never`.
    pub(crate) use_color: bool,

    /// On `false`, pass `--no-deps` to `cargo`.
    pub(crate) deps: bool,
}

impl GenerationSettings {
    fn stderr(&self) -> std::process::Stdio {
        if self.pass_through_stderr {
            // Print cargo update progress
            std::process::Stdio::inherit()
        } else {
            std::process::Stdio::piped()
        }
    }

    fn color_flag(&self) -> &'static str {
        if self.use_color {
            "--color=always"
        } else {
            "--color=never"
        }
    }
}

pub(super) fn generate_rustdoc(
    request: &CrateDataRequest<'_>,
    build_dir: &Path,
    cargo_target_dir: &Path,
    settings: GenerationSettings,
    callbacks: &mut CallbackHandler<'_>,
) -> Result<(PathBuf, cargo_metadata::Metadata), TerminalError> {
    let crate_name = request.kind.name().into_terminal_result()?;
    let version = request.kind.version().into_terminal_result()?;

    // Generate an empty placeholder project with a dependency on the crate
    // whose rustdoc we need. We take this indirect generation path to avoid issues like:
    // https://github.com/obi1kenobi/cargo-semver-checks/issues/167#issuecomment-1382367128
    callbacks.generate_placeholder_project_start();
    let placeholder_manifest = create_placeholder_rustdoc_manifest(request)
        .context("failed to create placeholder manifest")
        .into_terminal_result()?;
    let placeholder_manifest_path =
        save_placeholder_rustdoc_manifest(build_dir, placeholder_manifest)
            .context("failed to save placeholder rustdoc manifest")
            .into_terminal_result()?;
    callbacks.generate_placeholder_project_success();

    if matches!(request.kind, RequestKind::LocalProject(..)) {
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
        // There is a still-unresolved problem here, due to the fact that
        // cargo does not guarantee it'll update dependencies to a newer lockfile
        // if our dependency is using a path dependency. This bit us in this case:
        //   https://github.com/obi1kenobi/cargo-semver-checks/issues/167#issuecomment-2324959305
        // That issue is tracked here: https://github.com/obi1kenobi/cargo-semver-checks/issues/902
        match run_cargo_update(
            crate_name,
            version,
            request,
            placeholder_manifest_path.as_path(),
            &settings,
        ) {
            CargoUpdateResult::Success => {}
            CargoUpdateResult::IoError(e) => {
                let error = anyhow::Error::new(e)
                    .context("IO error while running 'cargo update' on placeholder project");
                return Err(TerminalError::Other(error));
            }
            CargoUpdateResult::ErrorReturned(_exit_status, message) => {
                let error = anyhow::anyhow!(
                    "aborting due to failure to run 'cargo update' for crate {crate_name} v{version}"
                );
                return Err(TerminalError::WithAdvice(error, message));
            }
        }
    }

    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(&placeholder_manifest_path)
        .exec()?;

    let rustdoc_data = run_cargo_doc(
        request,
        &metadata,
        &placeholder_manifest_path,
        cargo_target_dir,
        crate_name,
        version,
        &settings,
        callbacks,
    )?;

    Ok((rustdoc_data, metadata))
}

fn produce_repro_workspace_shell_commands(request: &CrateDataRequest<'_>) -> String {
    let selector = match &request.kind {
        RequestKind::Registry { .. } => format!(
            "{}@={}",
            request.kind.name().expect("failed to get crate name"),
            request.kind.version().expect("failed to get crate version")
        ),
        RequestKind::LocalProject(project) => format!(
            "--path {}",
            project
                .manifest
                .path
                .parent()
                .expect("source Cargo.toml had no parent path")
                .canonicalize()
                .expect("failed to canonicalize path")
                .to_str()
                .expect("failed to create path string")
        ),
    };
    let no_default_features = if !request.default_features {
        "--no-default-features "
    } else {
        ""
    };
    let feature_list = if request.extra_features.is_empty() {
        "".to_string()
    } else {
        format!("--features {} ", request.extra_features.iter().join(","))
    };
    format!(
        "      \
    cargo new --lib example &&
          cd example &&
          echo '[workspace]' >> Cargo.toml &&
          cargo add {selector} {no_default_features}{feature_list}&&
          "
    )
}

enum CargoUpdateResult {
    Success,
    IoError(std::io::Error),
    ErrorReturned(std::process::ExitStatus, String),
}

/// Determine the default target triple configured in the environment.
fn get_default_build_target_triple(config: &cargo_config2::Config) -> anyhow::Result<String> {
    // If `CARGO_BUILD_TARGET` is set, it dominates other options.
    if let Ok(value) = std::env::var("CARGO_BUILD_TARGET") {
        return Ok(value);
    }

    // `build.target` from (merged) cargo config following its precedence rules is next.
    //
    // Cargo supports multiple build targets, but `cargo-semver-checks` does not.
    // If the cargo config sets multiple targets, we only look at the first one.
    // Tracking issue: https://github.com/obi1kenobi/cargo-semver-checks/issues/1470
    if let Some(first_build_target) = config
        .build
        .target
        .iter()
        .flat_map(|triples| triples.iter())
        .next()
    {
        return Ok(first_build_target.triple().to_string());
    }

    // No build target set in cargo config. We're using the implied host target then,
    // ask rustc what that is:
    // ```
    // $ rustc --print host-tuple
    // x86_64-unknown-linux-gnu
    // ```
    let cmd_output = std::process::Command::new("rustc")
        .args(["--print", "host-tuple"])
        .output()
        .context("'rustc --print host-tuple' failed, is Rust installed correctly?")?;

    let mut triple = String::from_utf8(cmd_output.stdout)
        .context("'rustc --print host-tuple' output is not legal UTF-8")?;
    triple.truncate(triple.trim_end().len());

    Ok(triple)
}

struct Flags {
    rustflags: String,
    rustdocflags: String,
}

fn decode_cargo_flags(encoded: String) -> String {
    const SEPARATOR: char = char::from_u32(0x1f).unwrap();
    if !encoded.contains(SEPARATOR) {
        // Only a single flag, nothing to do.
        return encoded;
    }

    encoded.split(SEPARATOR).join(" ")
}

impl Flags {
    // Matching cargo's behavior:
    // https://doc.rust-lang.org/cargo/reference/config.html#buildrustflags
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html
    fn determine_rustflags(
        config: &cargo_config2::Config,
        target_triple: &str,
    ) -> anyhow::Result<String> {
        // First, check env vars.
        // `CARGO_BUILD_RUSTFLAGS` or `CARGO_ENCODED_RUSTFLAGS` or `RUSTFLAGS`
        if let Ok(rustflags) = std::env::var("CARGO_BUILD_RUSTFLAGS") {
            return Ok(rustflags);
        }
        if let Ok(encoded_rustflags) = std::env::var("CARGO_ENCODED_RUSTFLAGS") {
            // Flags are separated by `0x1f` (ASCII Unit Separator).
            // https://doc.rust-lang.org/cargo/reference/environment-variables.html
            //
            // HACK(predrag): We decode the flags for now. This is arguably incorrect,
            // and we should maybe instead encode all our flags to the `0x1f`-separated form.
            return Ok(decode_cargo_flags(encoded_rustflags));
        }
        if let Ok(rustflags) = std::env::var("RUSTFLAGS") {
            return Ok(rustflags);
        }

        // Nothing in env vars. Per the cargo docs, the next option is:
        // All matching `target.<triple>.rustflags` and `target.<cfg>.rustflags` config entries
        // joined together.
        if let Some(flags) = config.rustflags(target_triple)? {
            return Ok(flags.encode_space_separated()?);
        }

        // Nothing in target-specific rustflags. Per the cargo docs,
        // the last option is `build.rustflags`.
        Ok(config
            .build
            .rustflags
            .as_ref()
            .map(|flags| flags.encode_space_separated())
            .transpose()?
            .unwrap_or_default())
    }

    // Matching cargo's behavior:
    // https://doc.rust-lang.org/cargo/reference/config.html#buildrustdocflags
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html
    fn determine_rustdocflags(
        config: &cargo_config2::Config,
        target_triple: &str,
    ) -> anyhow::Result<String> {
        // First, check env vars.
        // `CARGO_BUILD_RUSTDOCFLAGS` or `CARGO_ENCODED_RUSTDOCFLAGS` or `RUSTDOCFLAGS`
        if let Ok(rustdocflags) = std::env::var("CARGO_BUILD_RUSTDOCFLAGS") {
            return Ok(rustdocflags);
        }
        if let Ok(encoded_rustdocflags) = std::env::var("CARGO_ENCODED_RUSTDOCFLAGS") {
            // Flags are separated by `0x1f` (ASCII Unit Separator).
            // https://doc.rust-lang.org/cargo/reference/environment-variables.html
            //
            // HACK(predrag): We decode the flags for now. This is arguably incorrect,
            // and we should maybe instead encode all our flags to the `0x1f`-separated form.
            return Ok(decode_cargo_flags(encoded_rustdocflags));
        }
        if let Ok(rustdocflags) = std::env::var("RUSTDOCFLAGS") {
            return Ok(rustdocflags);
        }

        // Nothing in env vars. Per the cargo docs, the next option is:
        // All matching `target.<triple>.rustdocflags` config entries joined together.
        if let Some(flags) = config.rustdocflags(target_triple)? {
            return Ok(flags.encode_space_separated()?);
        }

        // Nothing in target-specific rustdocflags. Per the cargo docs,
        // the last option is `build.rustdocflags`.
        Ok(config
            .build
            .rustdocflags
            .as_ref()
            .map(|flags| flags.encode_space_separated())
            .transpose()?
            .unwrap_or_default())
    }

    fn from_env_and_config() -> anyhow::Result<Self> {
        let config = cargo_config2::Config::load().context("failed to inspect cargo config")?;
        let triple = get_default_build_target_triple(&config)?;

        Ok(Self {
            rustflags: Self::determine_rustflags(&config, &triple)?,
            rustdocflags: Self::determine_rustdocflags(&config, &triple)?,
        })
    }
}

/// Run `cargo update` inside a placeholder workspace.
fn run_cargo_update(
    crate_name: &str,
    version: &str,
    request: &CrateDataRequest<'_>,
    placeholder_manifest_path: &Path,
    settings: &GenerationSettings,
) -> CargoUpdateResult {
    let mut cmd = std::process::Command::new("cargo");
    cmd.stdout(std::process::Stdio::null()) // Don't pollute output
        .stderr(settings.stderr())
        .arg("update")
        .arg("--manifest-path")
        .arg(placeholder_manifest_path);

    // Respect our configured color choice.
    cmd.arg(settings.color_flag());

    let output = match cmd.output() {
        Ok(output) => output,
        Err(e) => return CargoUpdateResult::IoError(e),
    };

    if !output.status.success() {
        let mut message = String::with_capacity(1024);
        if settings.pass_through_stderr {
            writeln!(message, "error: running 'cargo update' on crate '{crate_name}' v{version} failed, see stderr output above").expect("formatting failed");
        } else {
            let delimiter = "-----";
            writeln!(
                message,
                "\
                error: running 'cargo update' on crate '{crate_name}' failed with output:\n\
                {delimiter}\n{}\n{delimiter}\n\
                error: failed to update dependencies for crate {crate_name} v{version}",
                String::from_utf8_lossy(&output.stderr)
            )
            .expect("formatting failed");
        }
        writeln!(
            message,
            "note: this is unlikely to be a bug in cargo-semver-checks,"
        )
        .expect("formatting failed");
        writeln!(
            message,
            "      and is probably an issue with the crate's Cargo.toml"
        )
        .expect("formatting failed");
        writeln!(
            message,
            "note: the following command can be used to reproduce the compilation error:"
        )
        .expect("formatting failed");
        let repro_base = produce_repro_workspace_shell_commands(request);
        writeln!(message, "{repro_base}cargo update").expect("formatting failed");

        return CargoUpdateResult::ErrorReturned(output.status, message);
    }

    CargoUpdateResult::Success
}

#[allow(clippy::too_many_arguments)]
fn run_cargo_doc(
    request: &CrateDataRequest<'_>,
    metadata: &cargo_metadata::Metadata,
    placeholder_manifest_path: &Path,
    target_dir: &Path,
    crate_name: &str,
    version: &str,
    settings: &GenerationSettings,
    callbacks: &mut CallbackHandler<'_>,
) -> Result<PathBuf, TerminalError> {
    let pkg_spec = format!("{crate_name}@{version}");

    // Load the current rustflags and rustdocflags settings.
    // We'll want to modify them.
    let Flags {
        rustflags: mut prior_rustflags,
        rustdocflags: mut prior_rustdocflags,
    } = Flags::from_env_and_config().into_terminal_result()?;

    // Generating rustdoc JSON for a crate also involves checking that crate's dependencies.
    // The check is done by rustc, not rustdoc, so it's subject to `RUSTFLAGS` not `RUSTDOCFLAGS`.
    // We don't want rustc to fail that check if the user has set `RUSTFLAGS="-Dwarnings"` here.
    // This fixes: https://github.com/obi1kenobi/cargo-semver-checks/issues/589
    let rustflags = if !prior_rustflags.is_empty() {
        prior_rustflags.push_str(" --cap-lints=allow");
        std::borrow::Cow::Owned(prior_rustflags)
    } else {
        std::borrow::Cow::Borrowed("--cap-lints=allow")
    };

    // Ensure we preserve `RUSTDOCFLAGS` if they are set.
    // This allows users to supply `--cfg <custom-value>` settings in `RUSTDOCFLAGS`
    // in order to toggle what functionality is compiled into the scanned crate.
    // Suggested in: https://github.com/obi1kenobi/cargo-semver-checks/discussions/1012
    let extra_rustdocflags = "-Z unstable-options --document-private-items --document-hidden-items --output-format=json --cap-lints=allow";
    let rustdocflags = if !prior_rustdocflags.is_empty() {
        prior_rustdocflags.push(' ');
        prior_rustdocflags.push_str(extra_rustdocflags);
        std::borrow::Cow::Owned(prior_rustdocflags)
    } else {
        std::borrow::Cow::Borrowed(extra_rustdocflags)
    };

    // Run the rustdoc generation command on the placeholder crate,
    // specifically requesting the rustdoc of *only* the crate specified in `pkg_spec`.
    //
    // N.B.: Passing `--all-features` here would have no effect, since that only applies to
    //       the top-level project i.e. the placeholder, which has no features.
    //       To generate rustdoc for our intended crate with features enabled,
    //       those features must be enabled on the dependency in the `Cargo.toml`
    //       of the placeholder project.
    callbacks.generate_rustdoc_start();
    let mut cmd = std::process::Command::new("cargo");
    cmd.env("RUSTC_BOOTSTRAP", "1")
        .env("RUSTDOCFLAGS", rustdocflags.as_ref())
        .env("RUSTFLAGS", rustflags.as_ref())
        .stdout(std::process::Stdio::null()) // Don't pollute output
        .stderr(settings.stderr())
        .arg("doc")
        .arg("--manifest-path")
        .arg(placeholder_manifest_path)
        .arg("--target-dir")
        .arg(target_dir)
        .arg("--package")
        .arg(pkg_spec)
        .arg("--lib");
    if let Some(build_target) = request.build_target {
        cmd.arg("--target").arg(build_target);
    }
    if !settings.deps {
        cmd.arg("--no-deps");
    }

    // Respect our configured color choice
    cmd.arg(settings.color_flag());

    let output = cmd.output()?;
    if !output.status.success() {
        let mut message = String::with_capacity(1024);
        if settings.pass_through_stderr {
            writeln!(message, "error: running cargo-doc on crate {crate_name} v{version} failed, see stderr output above").expect("formatting failed");
        } else {
            let delimiter = "-----";
            writeln!(
                message,
                "error: running cargo-doc on crate '{crate_name}' failed with output:"
            )
            .expect("formatting failed");
            writeln!(
                message,
                "{delimiter}\n{}\n{delimiter}\n",
                String::from_utf8_lossy(&output.stderr)
            )
            .expect("formatting failed");
            writeln!(
                message,
                "error: failed to build rustdoc for crate {crate_name} v{version}"
            )
            .expect("formatting failed");
        }

        writeln!(
            message,
            "note: this is usually due to a compilation error in the crate,"
        )
        .expect("formatting failed");
        writeln!(
            message,
            "      and is unlikely to be a bug in cargo-semver-checks"
        )
        .expect("formatting failed");

        if rustflags.contains("--cfg") {
            writeln!(
                message,
                "note: RUSTFLAGS appears to contain '--cfg' arguments."
            )
            .expect("formatting failed");
            writeln!(
                message,
                "      Rustdoc only uses '--cfg' options in RUSTDOCFLAGS."
            )
            .expect("formatting failed");
            writeln!(
                message,
                "      Setting the same flags in RUSTDOCFLAGS may resolve the problem."
            )
            .expect("formatting failed");
        }

        writeln!(
            message,
            "note: the following command can be used to reproduce the error:"
        )
        .expect("formatting failed");

        let repro_base = produce_repro_workspace_shell_commands(request);
        let build_target_flag = if let Some(build_target) = request.build_target {
            format!(" --target {build_target}")
        } else {
            String::new()
        };
        writeln!(
            message,
            "\
{repro_base}cargo check{build_target_flag} &&
          cargo doc{build_target_flag}"
        )
        .expect("formatting failed");

        return Err(TerminalError::WithAdvice(
            anyhow::anyhow!(
                "aborting due to failure to build rustdoc for crate {crate_name} v{version}"
            ),
            message,
        ));
    }

    let rustdoc_dir = determine_rustdoc_dir(request, target_dir, crate_name, version)?;

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
    let observed_stderr_but_lib_msg_not_present = if !settings.pass_through_stderr {
        let stderr_output = String::from_utf8_lossy(&output.stderr);
        !stderr_output.contains("ignoring invalid dependency ")
            || !stderr_output.contains(" which is missing a lib target")
    } else {
        false
    };
    let subject_crate = metadata
        .packages
        .iter()
        .find(|dep| dep.name.as_str() == crate_name)
        .ok_or_else(|| {
            if !observed_stderr_but_lib_msg_not_present {
                anyhow::anyhow!("crate {crate_name} v{version} has no lib target, nothing to check")
            } else {
                panic!(
                    "We declared a dependency on crate '{crate_name}', but it doesn't exist \
in the metadata and stderr didn't mention it was lacking a lib target. This is probably a bug.",
                );
            }
        })
        .into_terminal_result()?;

    // Figure out the name of the JSON file where rustdoc will produce the output we want.
    // The name is:
    // - the name of the library-like target of the crate, not the crate's name
    // - but with all `-` chars replaced with `_` instead.
    // Related: https://github.com/obi1kenobi/cargo-semver-checks/issues/432
    if let Some(lib_target) = subject_crate
        .targets
        .iter()
        .find(|target| crate::is_lib_like_checkable_target(target))
    {
        let lib_name = lib_target.name.as_str();
        let rustdoc_json_file_name = lib_name.replace('-', "_");

        let json_path = rustdoc_dir.join(format!("{rustdoc_json_file_name}.json"));
        if json_path.exists() {
            callbacks.generate_rustdoc_success();
            return Ok(json_path);
        } else {
            return Err(TerminalError::Other(anyhow::anyhow!(
                "could not find expected rustdoc output for `{}`: {}",
                crate_name,
                json_path.display()
            )));
        }
    }

    Err(TerminalError::Other(anyhow::anyhow!(
        "crate {crate_name} v{version} has no lib target, so there's nothing to check",
    )))
}

fn determine_rustdoc_dir(
    request: &CrateDataRequest<'_>,
    target_dir: &Path,
    crate_name: &str,
    version: &str,
) -> Result<PathBuf, TerminalError> {
    // If the build target is explicitly specified, the rustdoc JSON is inside its directory.
    if let Some(build_target) = request.build_target {
        return Ok(target_dir.join(build_target).join("doc"));
    }

    // Check if cargo might be configured with a build target different than the host.
    // Even if no explicit `--target` flag is passed, cargo may choose a different target
    // based on its config files and environment variables. The best way to check this
    // is to ask cargo itself.
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
            .expect("non-utf8 cargo output")
            // this is the only way to detect a not set config value currently:
            //      https://github.com/rust-lang/cargo/issues/13223
            .contains("config value `build.target` is not set")
        {
            None
        } else {
            let mut message = String::with_capacity(1024);
            let delimiter = "-----";
            writeln!(
                message,
                "error: running cargo-config on crate '{crate_name}' failed with output:"
            )
            .expect("formatting failed");
            writeln!(
                message,
                "{delimiter}\n{}\n{delimiter}\n",
                String::from_utf8_lossy(&output.stderr)
            )
            .expect("formatting failed");

            writeln!(
                message,
                "error: unexpected cargo config output for crate {crate_name} v{version}\n"
            )
            .expect("formatting failed");
            writeln!(
                message,
                "note: this may be a bug in cargo, or a bug in cargo-semver-checks;"
            )
            .expect("formatting failed");
            writeln!(
                message,
                "      if unsure, feel free to open a GitHub issue on cargo-semver-checks"
            )
            .expect("formatting failed");
            writeln!(
                message,
                "note: running the following command on the crate should reproduce the error:"
            )
            .expect("formatting failed");
            writeln!(
                message,
                "      cargo config -Zunstable-options get --format=json-value build.target",
            )
            .expect("formatting failed");

            return Err(TerminalError::WithAdvice(
                anyhow::anyhow!(
                    "aborting due to cargo-config failure on crate {crate_name} v{version}"
                ),
                message,
            ));
        }
    };

    let rustdoc_dir = if let Some(build_target) = build_target {
        target_dir.join(build_target).join("doc")
    } else {
        target_dir.join("doc")
    };
    Ok(rustdoc_dir)
}

/// To get the rustdoc of the project, we first create a placeholder project somewhere
/// with the project as a dependency, and run `cargo rustdoc` on it.
fn create_placeholder_rustdoc_manifest(
    request: &CrateDataRequest<'_>,
) -> anyhow::Result<cargo_toml::Manifest<()>> {
    use cargo_toml::*;

    Ok(Manifest::<()> {
        package: {
            let mut package = Package::new("placeholder", "0.0.0");
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
            let project_with_features: DependencyDetail = match &request.kind {
                RequestKind::Registry { .. } => DependencyDetail {
                    // We need the *exact* version as a dependency, or else cargo will
                    // give us the latest semver-compatible version which is not we want.
                    // Fixes: https://github.com/obi1kenobi/cargo-semver-checks/issues/261
                    version: Some(format!("={}", request.kind.version()?)),
                    default_features: request.default_features,
                    features: request
                        .extra_features
                        .iter()
                        .map(ToString::to_string)
                        .collect(),
                    ..DependencyDetail::default()
                },
                RequestKind::LocalProject(local_request) => {
                    DependencyDetail {
                        path: Some({
                            let dir_path = crate::manifest::get_project_dir_from_manifest_path(
                                &local_request.manifest.path,
                            )?;
                            // The manifest will be saved in some other directory,
                            // so for convenience, we're using absolute paths.
                            dir_path
                                .canonicalize()
                                .context("failed to canonicalize manifest path")?
                                .to_str()
                                .context("manifest path is not valid UTF-8")?
                                .to_string()
                        }),
                        features: request
                            .extra_features
                            .iter()
                            .map(ToString::to_string)
                            .collect(),
                        default_features: request.default_features,
                        ..DependencyDetail::default()
                    }
                }
            };

            let mut deps = DepsSet::new();
            deps.insert(
                request.kind.name()?.to_string(),
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
