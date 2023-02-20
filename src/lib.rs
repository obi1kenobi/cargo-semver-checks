#![forbid(unsafe_code)]

mod check_release;
mod config;
mod manifest;
mod query;
mod rustdoc_cmd;
mod rustdoc_gen;
mod templating;
mod util;

use anyhow::Context;
use cargo_metadata::PackageId;
use clap::ValueEnum;
pub use config::*;
use directories::ProjectDirs;
pub use query::*;

use check_release::run_check_release;
use trustfall_rustdoc::{load_rustdoc, VersionedCrate};

use itertools::Itertools;
use rustdoc_cmd::RustdocCommand;
use semver::Version;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Test a release for semver violations.
#[non_exhaustive]
#[derive(Debug)]
pub struct Check {
    /// Which packages to analyze.
    scope: Scope,
    current: Rustdoc,
    baseline: Rustdoc,
    log_level: Option<log::Level>,
    release_type: Option<ReleaseType>,
}

/// The kind of release we're making.
///
/// Affects which lints are executed.
/// Non-exhaustive in case we want to add "pre-release" as an option in the future.
#[non_exhaustive]
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseType {
    Major,
    Minor,
    Patch,
}

#[non_exhaustive]
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
    /// Same as [`Rustdoc::from_git_revision()`], but with the current git revision.
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
    pub fn from_registry_latest_crate_version() -> Self {
        Self {
            source: RustdocSource::VersionFromRegistry(None),
        }
    }

    /// Generate the rustdoc file from a specific crate version.
    pub fn from_registry(crate_version: impl Into<String>) -> Self {
        Self {
            source: RustdocSource::VersionFromRegistry(Some(crate_version.into())),
        }
    }
}

#[derive(Debug)]
enum RustdocSource {
    /// Path to the Rustdoc json file.
    /// Use this option when you have already generated the rustdoc file.
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
    VersionFromRegistry(Option<String>),
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

#[non_exhaustive]
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

#[non_exhaustive]
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
            baseline: Rustdoc::from_registry_latest_crate_version(),
            log_level: Default::default(),
            release_type: None,
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

    pub fn with_release_type(&mut self, release_type: ReleaseType) -> &mut Self {
        self.release_type = Some(release_type);
        self
    }

    /// Some `RustdocSource`s don't contain a path to the project root,
    /// so they don't have a target directory. We try to deduce the target directory
    /// on a "best effort" basis -- when the source contains a target dir,
    /// we use it, otherwise when the other source contains one, we use it,
    /// otherwise we just use a standard cache folder as specified by XDG.
    /// We cannot use a temporary directory, because the rustdocs from registry
    /// are being cached in the target directory.
    fn get_target_dir(&self, source: &RustdocSource) -> anyhow::Result<PathBuf> {
        Ok(
            if let Some(path) = get_target_dir_from_project_root(source)? {
                path
            } else if let Some(path) = get_target_dir_from_project_root(&self.current.source)? {
                path
            } else if let Some(path) = get_target_dir_from_project_root(&self.baseline.source)? {
                path
            } else {
                get_cache_dir()?
            },
        )
    }

    fn get_rustdoc_generator(
        &self,
        config: &mut GlobalConfig,
        source: &RustdocSource,
    ) -> anyhow::Result<Box<dyn rustdoc_gen::RustdocGenerator>> {
        let target_dir = self.get_target_dir(source)?;
        Ok(match source {
            RustdocSource::Rustdoc(path) => {
                Box::new(rustdoc_gen::RustdocFromFile::new(path.to_owned()))
            }
            RustdocSource::Root(root) => {
                Box::new(rustdoc_gen::RustdocFromProjectRoot::new(root, &target_dir)?)
            }
            RustdocSource::Revision(root, rev) => {
                let metadata = manifest_metadata_no_deps(root)?;
                let source = metadata.workspace_root.as_std_path();
                Box::new(rustdoc_gen::RustdocFromGitRevision::with_rev(
                    source,
                    &target_dir,
                    rev,
                    config,
                )?)
            }
            RustdocSource::VersionFromRegistry(version) => {
                let mut registry = rustdoc_gen::RustdocFromRegistry::new(&target_dir, config)?;
                if let Some(ver) = version {
                    let semver = semver::Version::parse(ver)?;
                    registry.set_version(semver);
                }
                Box::new(registry)
            }
        })
    }

    pub fn check_release(&self) -> anyhow::Result<Report> {
        let mut config = GlobalConfig::new().set_level(self.log_level);
        let rustdoc_cmd = RustdocCommand::new()
            .deps(false)
            .silence(!config.is_verbose());

        let current_loader = self.get_rustdoc_generator(&mut config, &self.current.source)?;
        let baseline_loader = self.get_rustdoc_generator(&mut config, &self.baseline.source)?;

        let all_outcomes: Vec<anyhow::Result<bool>> = match &self.current.source {
            RustdocSource::Rustdoc(_)
            | RustdocSource::Revision(_, _)
            | RustdocSource::VersionFromRegistry(_) => {
                let names = match &self.scope.mode {
                    ScopeMode::DenyList(_) =>
                        match &self.current.source {
                            RustdocSource::Rustdoc(_) =>
                                vec!["the-name-doesnt-matter-here".to_string()],
                            _ => panic!("couldn't deduce crate name, specify one through the package allow list")
                        }
                    ScopeMode::AllowList(lst) => lst.clone(),
                };
                names
                    .iter()
                    .map(|name| {
                        let version = None;
                        let (current_crate, baseline_crate) = generate_versioned_crates(
                            &mut config,
                            &rustdoc_cmd,
                            &*current_loader,
                            &*baseline_loader,
                            name,
                            version,
                        )?;

                        let success = run_check_release(
                            &mut config,
                            name,
                            current_crate,
                            baseline_crate,
                            self.release_type,
                        )?;
                        Ok(success)
                    })
                    .collect()
            }
            RustdocSource::Root(project_root) => {
                let metadata = manifest_metadata(project_root)?;
                let selected = self.scope.selected_packages(&metadata);
                selected
                    .iter()
                    .map(|selected| {
                        let crate_name = &selected.name;
                        let version = &selected.version;

                        let is_implied = matches!(
                            self.scope.mode,
                            ScopeMode::DenyList(PackageSelection {
                                selection: ScopeSelection::Workspace,
                                ..
                            })
                        ) && selected.publish == Some(vec![]);
                        if is_implied {
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
                                &rustdoc_cmd,
                                &*current_loader,
                                &*baseline_loader,
                                crate_name,
                                Some(version),
                            )?;

                            Ok(run_check_release(
                                &mut config,
                                crate_name,
                                current_crate,
                                baseline_crate,
                                self.release_type,
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
}

#[non_exhaustive]
#[derive(Debug)]
pub struct Report {
    success: bool,
}

impl Report {
    pub fn success(&self) -> bool {
        self.success
    }
}

fn generate_versioned_crates(
    config: &mut GlobalConfig,
    rustdoc_cmd: &RustdocCommand,
    current_loader: &dyn rustdoc_gen::RustdocGenerator,
    baseline_loader: &dyn rustdoc_gen::RustdocGenerator,
    crate_name: &str,
    version: Option<&Version>,
) -> anyhow::Result<(VersionedCrate, VersionedCrate)> {
    let current_path = current_loader.load_rustdoc(
        config,
        rustdoc_cmd,
        rustdoc_gen::CrateDataForRustdoc {
            name: crate_name,
            crate_type: rustdoc_gen::CrateType::Current,
        },
    )?;
    let current_crate = load_rustdoc(&current_path)?;

    // The process of generating baseline rustdoc can overwrite
    // the already-generated rustdoc of the current crate.
    // For example, this happens when target-dir is specified in `.cargo/config.toml`.
    // That's the reason why we're immediately loading the rustdocs into memory.
    // See: https://github.com/obi1kenobi/cargo-semver-checks/issues/269
    let baseline_path = baseline_loader.load_rustdoc(
        config,
        rustdoc_cmd,
        rustdoc_gen::CrateDataForRustdoc {
            name: crate_name,
            crate_type: rustdoc_gen::CrateType::Baseline {
                highest_allowed_version: version,
            },
        },
    )?;
    let baseline_crate = load_rustdoc(&baseline_path)?;

    Ok((current_crate, baseline_crate))
}

fn manifest_path(project_root: &Path) -> anyhow::Result<PathBuf> {
    if project_root.is_dir() {
        let manifest_path = project_root.join("Cargo.toml");
        // Checking whether the file exists here is not necessary
        // (it will nevertheless be checked while parsing the manifest),
        // but it should give a nicer error message for the user.
        if manifest_path.exists() {
            Ok(manifest_path)
        } else {
            anyhow::bail!(
                "couldn't find Cargo.toml in directory {}",
                project_root.display()
            )
        }
    } else if project_root.ends_with("Cargo.toml") {
        // Even though the `project_root` should be a directory,
        // someone could by accident directly pass the path to the manifest
        // and we're kind enough to accept it.
        Ok(project_root.to_path_buf())
    } else {
        anyhow::bail!(
            "path {} is not a directory or a manifest",
            project_root.display()
        )
    }
}

fn manifest_metadata(project_root: &Path) -> anyhow::Result<cargo_metadata::Metadata> {
    let manifest_path = manifest_path(project_root)?;
    let mut command = cargo_metadata::MetadataCommand::new();
    let metadata = command.manifest_path(manifest_path).exec()?;
    Ok(metadata)
}

fn manifest_metadata_no_deps(project_root: &Path) -> anyhow::Result<cargo_metadata::Metadata> {
    let manifest_path = manifest_path(project_root)?;
    let mut command = cargo_metadata::MetadataCommand::new();
    let metadata = command.manifest_path(manifest_path).no_deps().exec()?;
    Ok(metadata)
}

fn get_cache_dir() -> anyhow::Result<PathBuf> {
    let project_dirs =
        ProjectDirs::from("", "", "cargo-semver-checks").context("can't determine project dirs")?;
    let cache_dir = project_dirs.cache_dir();
    fs::create_dir_all(cache_dir).context("can't create cache dir")?;
    Ok(cache_dir.to_path_buf())
}

fn get_target_dir_from_project_root(source: &RustdocSource) -> anyhow::Result<Option<PathBuf>> {
    Ok(match source {
        RustdocSource::Root(root) => {
            let metadata = manifest_metadata_no_deps(root)?;
            let target = metadata.target_directory.as_std_path().join(util::SCOPE);
            Some(target)
        }
        RustdocSource::Revision(root, rev) => {
            let metadata = manifest_metadata_no_deps(root)?;
            let target = metadata.target_directory.as_std_path().join(util::SCOPE);
            let target = target.join(format!("git-{}", util::slugify(rev)));
            Some(target)
        }
        RustdocSource::Rustdoc(_path) => None,
        RustdocSource::VersionFromRegistry(_version) => None,
    })
}
