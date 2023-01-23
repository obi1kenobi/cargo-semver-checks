#![forbid(unsafe_code)]

mod baseline;
mod check_release;
mod config;
mod dump;
mod manifest;
mod query;
mod templating;
mod util;

pub use config::*;
pub use query::*;

use check_release::run_check_release;
use trustfall_rustdoc::{load_rustdoc, VersionedCrate};

use dump::RustDocCommand;
use itertools::Itertools;
use semver::Version;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Test a release for semver violations.
#[derive(Default)]
pub struct Check {
    /// Which packages to analyze.
    scope: Scope,
    current: Current,
    baseline: Baseline,
    log_level: Option<log::Level>,
}

enum Baseline {
    /// Version from registry to lookup for a baseline. E.g. "1.0.0".
    /// If `None`, uses the largest-numbered non-yanked non-prerelease version
    /// published to the cargo registry. If no such version, uses
    /// the largest-numbered version including yanked and prerelease versions.
    Version(Option<String>),
    /// Git revision to lookup for a baseline.
    Revision(String),
    /// Directory containing baseline crate source.
    Root(PathBuf),
    /// The rustdoc json file to use as a semver baseline.
    RustDoc(PathBuf),
}

impl Default for Baseline {
    fn default() -> Self {
        Self::Version(None)
    }
}

/// Current version of the project to analyze.
#[derive(Default)]
enum Current {
    /// Path to the manifest of the current version of the project.
    /// It can be a workspace or a single package.
    Manifest(PathBuf),
    /// The rustdoc json of the current version of the project.
    RustDoc(PathBuf),
    /// Use the manifest in the current directory.
    #[default]
    CurrentDir,
}

#[derive(Default)]
struct Scope {
    selection: ScopeSelection,
    excluded_packages: Vec<String>,
}

/// Which packages to analyze.
#[derive(Default, PartialEq, Eq)]
enum ScopeSelection {
    /// Package to process (see `cargo help pkgid`)
    Packages(Vec<String>),
    /// All packages in the workspace. Equivalent to `--workspace`.
    Workspace,
    /// Default members of the workspace.
    #[default]
    DefaultMembers,
}

impl Scope {
    fn selected_packages<'m>(
        &self,
        meta: &'m cargo_metadata::Metadata,
    ) -> Vec<&'m cargo_metadata::Package> {
        let workspace_members: HashSet<_> = meta.workspace_members.iter().collect();
        let base_ids: HashSet<_> = match &self.selection {
            ScopeSelection::DefaultMembers => {
                // Deviating from cargo because Metadata doesn't have default members
                let resolve = meta.resolve.as_ref().expect("no-deps is unsupported");
                match &resolve.root {
                    Some(root) => {
                        let mut base_ids = HashSet::new();
                        base_ids.insert(root);
                        base_ids
                    }
                    None => workspace_members,
                }
            }
            ScopeSelection::Workspace => workspace_members,
            ScopeSelection::Packages(patterns) => {
                meta.packages
                    .iter()
                    // Deviating from cargo by not supporting patterns
                    // Deviating from cargo by only checking workspace members
                    .filter(|p| workspace_members.contains(&p.id) && patterns.contains(&p.name))
                    .map(|p| &p.id)
                    .collect()
            }
        };

        meta.packages
            .iter()
            .filter(|p| base_ids.contains(&p.id) && !self.excluded_packages.contains(&p.name))
            .collect()
    }
}

impl Check {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_manifest(&mut self, path: PathBuf) -> &mut Self {
        self.current = Current::Manifest(path);
        self
    }

    pub fn with_workspace(&mut self) -> &mut Self {
        self.scope.selection = ScopeSelection::Workspace;
        self
    }

    pub fn with_packages(&mut self, packages: Vec<String>) -> &mut Self {
        self.scope.selection = ScopeSelection::Packages(packages);
        self
    }

    pub fn with_excluded_packages(&mut self, excluded_packages: Vec<String>) -> &mut Self {
        self.scope.excluded_packages = excluded_packages;
        self
    }

    pub fn with_current_rustdoc(&mut self, rustdoc: PathBuf) -> &mut Self {
        self.current = Current::RustDoc(rustdoc);
        self
    }

    pub fn with_baseline_version(&mut self, version: String) -> &mut Self {
        self.baseline = Baseline::Version(Some(version));
        self
    }

    pub fn with_baseline_revision(&mut self, revision: String) -> &mut Self {
        self.baseline = Baseline::Revision(revision);
        self
    }

    pub fn with_baseline_root(&mut self, root: PathBuf) -> &mut Self {
        self.baseline = Baseline::Root(root);
        self
    }

    pub fn with_baseline_rustdoc(&mut self, rustdoc: PathBuf) -> &mut Self {
        self.baseline = Baseline::RustDoc(rustdoc);
        self
    }

    pub fn with_log_level(&mut self, log_level: log::Level) -> &mut Self {
        self.log_level = Some(log_level);
        self
    }

    fn manifest_path(&self) -> anyhow::Result<PathBuf> {
        let path = match &self.current {
            Current::Manifest(path) => path.clone(),
            Current::RustDoc(_) => {
                anyhow::bail!("error: RustDoc is not supported with these arguments.")
            }
            Current::CurrentDir => PathBuf::from("Cargo.toml"),
        };
        Ok(path)
    }

    fn manifest_metadata(&self) -> anyhow::Result<cargo_metadata::Metadata> {
        let mut command = cargo_metadata::MetadataCommand::new();
        let metadata = command.manifest_path(self.manifest_path()?).exec()?;
        Ok(metadata)
    }

    fn manifest_metadata_no_deps(&self) -> anyhow::Result<cargo_metadata::Metadata> {
        let mut command = cargo_metadata::MetadataCommand::new();
        let metadata = command
            .manifest_path(self.manifest_path()?)
            .no_deps()
            .exec()?;
        Ok(metadata)
    }

    pub fn check_release(&self) -> anyhow::Result<Report> {
        let mut config = GlobalConfig::new().set_level(self.log_level);

        let loader: Box<dyn baseline::BaselineLoader> = match &self.baseline {
            Baseline::RustDoc(path) => Box::new(baseline::RustdocBaseline::new(path.to_owned())),
            Baseline::Root(root) => Box::new(baseline::PathBaseline::new(root)?),
            Baseline::Revision(rev) => {
                let metadata = self.manifest_metadata_no_deps()?;
                let source = metadata.workspace_root.as_std_path();
                let slug = util::slugify(rev);
                let target = metadata
                    .target_directory
                    .as_std_path()
                    .join(util::SCOPE)
                    .join(format!("git-{slug}"));
                Box::new(baseline::GitBaseline::with_rev(
                    source,
                    &target,
                    rev,
                    &mut config,
                )?)
            }
            Baseline::Version(version) => {
                let mut registry = self.registry_baseline(&mut config)?;
                if let Some(ver) = version {
                    let semver = semver::Version::parse(ver)?;
                    registry.set_version(semver);
                }
                Box::new(registry)
            }
        };
        let rustdoc_cmd = dump::RustDocCommand::new()
            .deps(false)
            .silence(!config.is_verbose());

        let all_outcomes: Vec<anyhow::Result<bool>> = match &self.current {
            Current::RustDoc(current_rustdoc_path) => {
                let name = "<unknown>";
                let version = None;
                let (current_crate, baseline_crate) = generate_versioned_crates(
                    &mut config,
                    CurrentCratePath::CurrentRustdocPath(current_rustdoc_path),
                    &*loader,
                    &rustdoc_cmd,
                    name,
                    version,
                )?;

                let success = run_check_release(&mut config, name, current_crate, baseline_crate)?;
                vec![Ok(success)]
            }
            Current::CurrentDir | Current::Manifest(_) => {
                let metadata = self.manifest_metadata()?;
                let selected = self.scope.selected_packages(&metadata);
                selected
                    .iter()
                    .map(|selected| {
                        let manifest_path = selected.manifest_path.as_std_path();
                        let crate_name = &selected.name;
                        let version = &selected.version;

                        let is_implied = self.scope.selection == ScopeSelection::Workspace;
                        if is_implied && selected.publish == Some(vec![]) {
                            config.verbose(|config| {
                                config.shell_status(
                                    "Skipping",
                                    format_args!("{crate_name} v{version} (current)"),
                                )
                            })?;
                            Ok(true)
                        } else {
                            config.shell_status(
                                "Parsing",
                                format_args!("{crate_name} v{version} (current)"),
                            )?;

                            let (current_crate, baseline_crate) = generate_versioned_crates(
                                &mut config,
                                CurrentCratePath::ManifestPath(manifest_path),
                                &*loader,
                                &rustdoc_cmd,
                                crate_name,
                                Some(version),
                            )?;

                            Ok(run_check_release(
                                &mut config,
                                crate_name,
                                current_crate,
                                baseline_crate,
                            )?)
                        }
                    })
                    .collect()
            }
        };
        let success = all_outcomes
            .into_iter()
            .fold_ok(true, std::ops::BitAnd::bitand)?;

        Ok(Report { success })
    }

    fn registry_baseline(
        &self,
        config: &mut GlobalConfig,
    ) -> Result<baseline::RegistryBaseline, anyhow::Error> {
        let metadata = self.manifest_metadata_no_deps()?;
        let target = metadata.target_directory.as_std_path().join(util::SCOPE);
        let registry = baseline::RegistryBaseline::new(&target, config)?;
        Ok(registry)
    }
}

pub struct Report {
    success: bool,
}

impl Report {
    pub fn success(&self) -> bool {
        self.success
    }
}

// Argument to the generate_versioned_crates function.
enum CurrentCratePath<'a> {
    CurrentRustdocPath(&'a Path), // If rustdoc is passed, it is just loaded into the memory.
    ManifestPath(&'a Path),       // Otherwise, the function generates the rustdoc.
}

fn generate_versioned_crates(
    config: &mut GlobalConfig,
    current_crate_path: CurrentCratePath,
    loader: &dyn baseline::BaselineLoader,
    rustdoc_cmd: &RustDocCommand,
    crate_name: &str,
    version: Option<&Version>,
) -> anyhow::Result<(VersionedCrate, VersionedCrate)> {
    let current_crate = match current_crate_path {
        CurrentCratePath::CurrentRustdocPath(rustdoc_path) => load_rustdoc(rustdoc_path)?,
        CurrentCratePath::ManifestPath(manifest_path) => {
            let rustdoc_path = rustdoc_cmd.dump(manifest_path, None, true)?;
            load_rustdoc(&rustdoc_path)?
        }
    };

    // The process of generating baseline rustdoc can overwrite
    // the already-generated rustdoc of the current crate.
    // For example, this happens when target-dir is specified in `.cargo/config.toml`.
    // That's the reason why we're immediately loading the rustdocs into memory.
    // See: https://github.com/obi1kenobi/cargo-semver-checks/issues/269
    let baseline_path = loader.load_rustdoc(config, rustdoc_cmd, crate_name, version)?;
    let baseline_crate = load_rustdoc(&baseline_path)?;

    Ok((current_crate, baseline_crate))
}
