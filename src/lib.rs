#![forbid(unsafe_code)]

mod baseline;
mod check_release;
mod config;
mod dump;
mod manifest;
mod query;
mod templating;
mod util;

use cargo_metadata::PackageId;
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
#[derive(Debug)]
pub struct Check {
    /// Which packages to analyze.
    scope: Scope,
    current: Rustdoc,
    baseline: Rustdoc,
    log_level: Option<log::Level>,
}

#[derive(Debug)]
pub struct Rustdoc {
    source: RustdocSource,
}

impl Rustdoc {
    /// Use an existing rustdoc file.
    pub fn from_path(rustdoc_path: impl Into<PathBuf>) -> Self {
        Self {
            source: RustdocSource::Rustdoc(rustdoc_path.into()),
        }
    }

    /// Generate the rustdoc file from the project root directory,
    /// i.e. the directory containing the crate source.
    /// It can be a workspace or a single package.
    /// Same as `from_git_revision`, but with the current git revision.
    pub fn from_root(project_root: impl Into<PathBuf>) -> Self {
        Self {
            source: RustdocSource::Root(project_root.into()),
        }
    }

    /// Generate the rustdoc file from the project at a given git revision.
    pub fn from_git_revision(
        project_root: impl Into<PathBuf>,
        revision: impl Into<String>,
    ) -> Self {
        Self {
            source: RustdocSource::Revision(project_root.into(), revision.into()),
        }
    }

    /// Generate the rustdoc file from the largest-numbered non-yanked non-prerelease version
    /// published to the cargo registry. If no such version, uses
    /// the largest-numbered version including yanked and prerelease versions.
    pub fn from_latest_version() -> Self {
        Self {
            source: RustdocSource::Version(None),
        }
    }

    pub fn from_version(version: impl Into<String>) -> Self {
        Self {
            source: RustdocSource::Version(Some(version.into())),
        }
    }
}

#[derive(Debug)]
enum RustdocSource {
    /// Path to the Rustdoc json file. Use this option when you have already generated the rustdoc file.
    Rustdoc(PathBuf),
    /// Project root directory, i.e. the directory containing the crate source.
    /// It can be a workspace or a single package.
    Root(PathBuf),
    /// Project root directory and Git Revision.
    Revision(PathBuf, String),
    /// Version from cargo registry to lookup. E.g. "1.0.0".
    /// If `None`, uses the largest-numbered non-yanked non-prerelease version
    /// published to the cargo registry. If no such version, uses
    /// the largest-numbered version including yanked and prerelease versions.
    Version(Option<String>),
}

/// Which packages to analyze.
#[derive(Default, Debug)]
struct Scope {
    mode: ScopeMode,
}

#[derive(Debug)]
enum ScopeMode {
    /// All packages except the excluded ones.
    DenyList(PackageSelection),
    /// Packages to process (see `cargo help pkgid`)
    AllowList(Vec<String>),
}

impl Default for ScopeMode {
    fn default() -> Self {
        Self::DenyList(PackageSelection::default())
    }
}

#[derive(Default, Clone, Debug)]
pub struct PackageSelection {
    selection: ScopeSelection,
    excluded_packages: Vec<String>,
}

impl PackageSelection {
    pub fn new(selection: ScopeSelection) -> Self {
        Self {
            selection,
            excluded_packages: vec![],
        }
    }

    pub fn with_excluded_packages(&mut self, packages: Vec<String>) -> &mut Self {
        self.excluded_packages = packages;
        self
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum ScopeSelection {
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
        let workspace_members: HashSet<&PackageId> = meta.workspace_members.iter().collect();
        let base_ids: HashSet<&PackageId> = match &self.mode {
            ScopeMode::DenyList(PackageSelection {
                selection,
                excluded_packages,
            }) => {
                let packages = match selection {
                    ScopeSelection::Workspace => workspace_members,
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
                };

                packages
                    .iter()
                    .filter(|p| !excluded_packages.contains(&meta[p].name))
                    .copied()
                    .collect()
            }
            ScopeMode::AllowList(patterns) => {
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
            .filter(|&p| base_ids.contains(&p.id))
            .collect()
    }
}

impl Check {
    pub fn new(current: Rustdoc) -> Self {
        Self {
            scope: Scope::default(),
            current,
            baseline: Rustdoc::from_latest_version(),
            log_level: Default::default(),
        }
    }

    pub fn with_package_selection(&mut self, selection: PackageSelection) -> &mut Self {
        self.scope.mode = ScopeMode::DenyList(selection);
        self
    }

    pub fn with_packages(&mut self, packages: Vec<String>) -> &mut Self {
        self.scope.mode = ScopeMode::AllowList(packages);
        self
    }

    pub fn with_baseline(&mut self, baseline: Rustdoc) -> &mut Self {
        self.baseline = baseline;
        self
    }

    pub fn with_log_level(&mut self, log_level: log::Level) -> &mut Self {
        self.log_level = Some(log_level);
        self
    }

    pub fn check_release(&self) -> anyhow::Result<Report> {
        let mut config = GlobalConfig::new().set_level(self.log_level);

        let loader: Box<dyn baseline::BaselineLoader> = match &self.baseline.source {
            RustdocSource::Rustdoc(path) => {
                Box::new(baseline::RustdocBaseline::new(path.to_owned()))
            }
            RustdocSource::Root(root) => Box::new(baseline::PathBaseline::new(root)?),
            RustdocSource::Revision(root, rev) => {
                let metadata = manifest_metadata_no_deps(root)?;
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
            RustdocSource::Version(version) => {
                let mut registry = {
                    let manifest_dir = match &self.current.source {
                        RustdocSource::Root(manifest_dir)
                        | RustdocSource::Revision(manifest_dir, _) => manifest_dir,
                        RustdocSource::Version(_) | RustdocSource::Rustdoc(_) => {
                            anyhow::bail!("this combination of current and baseline sources isn't supported yet")
                        }
                    };
                    let metadata = manifest_metadata_no_deps(manifest_dir)?;
                    let target = metadata.target_directory.as_std_path().join(util::SCOPE);
                    baseline::RegistryBaseline::new(&target, &mut config)?
                };
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

        let all_outcomes: Vec<anyhow::Result<bool>> = match &self.current.source {
            RustdocSource::Rustdoc(current_rustdoc_path) => {
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
            RustdocSource::Root(project_root) => {
                let metadata = manifest_metadata(project_root)?;
                let selected = self.scope.selected_packages(&metadata);
                selected
                    .iter()
                    .map(|selected| {
                        let manifest_path = selected.manifest_path.as_std_path();
                        let crate_name = &selected.name;
                        let version = &selected.version;

                        let is_implied = matches!(
                            self.scope.mode,
                            ScopeMode::DenyList(PackageSelection {
                                selection: ScopeSelection::Workspace,
                                ..
                            })
                        ) && selected.publish == Some(vec![]);
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
            RustdocSource::Revision(_, _) | RustdocSource::Version(_) => todo!(),
        };
        let success = all_outcomes
            .into_iter()
            .fold_ok(true, std::ops::BitAnd::bitand)?;

        Ok(Report { success })
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

fn manifest_from_dir(manifest_dir: &Path) -> PathBuf {
    manifest_dir.join("Cargo.toml")
}

fn manifest_metadata(manifest_dir: &Path) -> anyhow::Result<cargo_metadata::Metadata> {
    let manifest_path = manifest_from_dir(manifest_dir);
    let mut command = cargo_metadata::MetadataCommand::new();
    let metadata = command.manifest_path(manifest_path).exec()?;
    Ok(metadata)
}

fn manifest_metadata_no_deps(manifest_dir: &Path) -> anyhow::Result<cargo_metadata::Metadata> {
    let manifest_path = manifest_from_dir(manifest_dir);
    let mut command = cargo_metadata::MetadataCommand::new();
    let metadata = command.manifest_path(manifest_path).no_deps().exec()?;
    Ok(metadata)
}
