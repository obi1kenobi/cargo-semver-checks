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
use directories::ProjectDirs;
use itertools::Itertools;

use check_release::run_check_release;
use rustdoc_gen::CrateDataForRustdoc;
use trustfall_rustdoc::{load_rustdoc, VersionedCrate};

use rustdoc_cmd::RustdocCommand;
use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

pub use config::GlobalConfig;
pub use query::{
    ActualSemverUpdate, LintLevel, OverrideMap, OverrideStack, QueryOverride, RequiredSemverUpdate,
    SemverQuery,
};

/// Test a release for semver violations.
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub struct Check {
    /// Which packages to analyze.
    scope: Scope,
    current: Rustdoc,
    baseline: Rustdoc,
    release_type: Option<ReleaseType>,
    current_feature_config: rustdoc_gen::FeatureConfig,
    baseline_feature_config: rustdoc_gen::FeatureConfig,
    /// Which `--target` to use, if unset pass no flag
    build_target: Option<String>,
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
#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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
#[derive(Default, Debug, PartialEq, Eq)]
struct Scope {
    mode: ScopeMode,
}

#[derive(Debug, PartialEq, Eq)]
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
#[derive(Default, Clone, Debug, PartialEq, Eq)]
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

    pub fn set_excluded_packages(&mut self, packages: Vec<String>) -> &mut Self {
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
    /// Returns `(selected, skipped)` packages
    fn selected_packages<'m>(
        &self,
        meta: &'m cargo_metadata::Metadata,
    ) -> (
        Vec<&'m cargo_metadata::Package>,
        Vec<&'m cargo_metadata::Package>,
    ) {
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
            .filter(|&p| {
                // The package has to not have been explicitly excluded
                base_ids.contains(&p.id)
            })
            .partition(|&p| p.targets.iter().any(is_lib_like_checkable_target))
    }
}

/// Is the specified target able to be semver-checked as a library, of any sort.
///
/// This is a broader definition than cargo's own "lib" definition, since we can also
/// semver-check rlib, dylib, and staticlib targets as well.
fn is_lib_like_checkable_target(target: &cargo_metadata::Target) -> bool {
    target.is_lib()
        || target
            .kind
            .iter()
            .any(|kind| matches!(kind.as_str(), "rlib" | "dylib" | "staticlib"))
}

impl Check {
    pub fn new(current: Rustdoc) -> Self {
        Self {
            scope: Scope::default(),
            current,
            baseline: Rustdoc::from_registry_latest_crate_version(),
            release_type: None,
            current_feature_config: rustdoc_gen::FeatureConfig::default_for_current(),
            baseline_feature_config: rustdoc_gen::FeatureConfig::default_for_baseline(),
            build_target: None,
        }
    }

    pub fn set_package_selection(&mut self, selection: PackageSelection) -> &mut Self {
        self.scope.mode = ScopeMode::DenyList(selection);
        self
    }

    pub fn set_packages(&mut self, packages: Vec<String>) -> &mut Self {
        self.scope.mode = ScopeMode::AllowList(packages);
        self
    }

    pub fn set_baseline(&mut self, baseline: Rustdoc) -> &mut Self {
        self.baseline = baseline;
        self
    }

    pub fn set_release_type(&mut self, release_type: ReleaseType) -> &mut Self {
        self.release_type = Some(release_type);
        self
    }

    pub fn with_only_explicit_features(&mut self) -> &mut Self {
        self.current_feature_config.features_group = rustdoc_gen::FeaturesGroup::None;
        self.baseline_feature_config.features_group = rustdoc_gen::FeaturesGroup::None;
        self
    }

    pub fn with_default_features(&mut self) -> &mut Self {
        self.current_feature_config.features_group = rustdoc_gen::FeaturesGroup::Default;
        self.baseline_feature_config.features_group = rustdoc_gen::FeaturesGroup::Default;
        self
    }

    pub fn with_heuristically_included_features(&mut self) -> &mut Self {
        self.current_feature_config.features_group = rustdoc_gen::FeaturesGroup::Heuristic;
        self.baseline_feature_config.features_group = rustdoc_gen::FeaturesGroup::Heuristic;
        self
    }

    pub fn with_all_features(&mut self) -> &mut Self {
        self.current_feature_config.features_group = rustdoc_gen::FeaturesGroup::All;
        self.baseline_feature_config.features_group = rustdoc_gen::FeaturesGroup::All;
        self
    }

    pub fn set_extra_features(
        &mut self,
        extra_current_features: Vec<String>,
        extra_baseline_features: Vec<String>,
    ) -> &mut Self {
        self.current_feature_config.extra_features = extra_current_features;
        self.baseline_feature_config.extra_features = extra_baseline_features;
        self
    }

    /// Set what `--target` to build the documentation with, by default will not pass any flag
    /// relying on the users cargo configuration.
    pub fn set_build_target(&mut self, build_target: String) -> &mut Self {
        self.build_target = Some(build_target);
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

    pub fn check_release(&self, config: &mut GlobalConfig) -> anyhow::Result<Report> {
        let rustdoc_cmd = RustdocCommand::new().deps(false).silence(config.is_info());

        // If both the current and baseline rustdoc are given explicitly as a file path,
        // we don't need to use the installed rustc, and this check can be skipped.
        if !(matches!(self.current.source, RustdocSource::Rustdoc(_))
            && matches!(self.baseline.source, RustdocSource::Rustdoc(_)))
        {
            let rustc_version_needed = config.minimum_rustc_version();
            match rustc_version::version() {
                Ok(rustc_version) => {
                    if rustc_version < *rustc_version_needed {
                        let help = "HELP: to use the latest rustc, run `rustup update stable && cargo +stable semver-checks <args>`";
                        anyhow::bail!("rustc version is not high enough: >={rustc_version_needed} needed, got {rustc_version}\n\n{help}");
                    }
                }
                Err(error) => {
                    let help = format!("HELP: to avoid errors please ensure rustc >={rustc_version_needed} is used");
                    config.shell_warn(format_args!(
                        "failed to determine the current rustc version: {error}\n\n{help}"
                    ))?;
                }
            };
        }

        let current_loader = self.get_rustdoc_generator(config, &self.current.source)?;
        let baseline_loader = self.get_rustdoc_generator(config, &self.baseline.source)?;

        // Create a report for each crate.
        // We want to run all the checks, even if one returns `Err`.
        let all_outcomes: Vec<anyhow::Result<(String, Option<CrateReport>)>> = match &self
            .current
            .source
        {
            RustdocSource::Rustdoc(_)
            | RustdocSource::Revision(_, _)
            | RustdocSource::VersionFromRegistry(_) => {
                let names = match &self.scope.mode {
                    ScopeMode::DenyList(_) =>
                        match &self.current.source {
                            RustdocSource::Rustdoc(_) =>
                                // This is a user-facing string.
                                // For example, it appears when two pre-generated rustdoc files
                                // are semver-checked against each other.
                                vec!["<unknown>".to_string()],
                            _ => panic!("couldn't deduce crate name, specify one through the package allow list")
                        }
                    ScopeMode::AllowList(lst) => lst.clone(),
                };
                names
                    .into_iter()
                    .map(|name| {
                        let start = std::time::Instant::now();
                        let version = None;
                        let (current_crate, baseline_crate) = generate_versioned_crates(
                            config,
                            &rustdoc_cmd,
                            &*current_loader,
                            &*baseline_loader,
                            CrateDataForRustdoc {
                                crate_type: rustdoc_gen::CrateType::Current,
                                name: &name,
                                feature_config: &self.current_feature_config,
                                build_target: self.build_target.as_deref(),
                            },
                            CrateDataForRustdoc {
                                crate_type: rustdoc_gen::CrateType::Baseline {
                                    highest_allowed_version: version,
                                },
                                name: &name,
                                feature_config: &self.baseline_feature_config,
                                build_target: self.build_target.as_deref(),
                            },
                        )?;

                        let report = run_check_release(
                            config,
                            &name,
                            current_crate,
                            baseline_crate,
                            self.release_type,
                            OverrideStack::new(),
                        )?;
                        config.shell_status(
                            "Finished",
                            format_args!("[{:>8.3}s] {name}", start.elapsed().as_secs_f32()),
                        )?;
                        Ok((name, Some(report)))
                    })
                    .collect()
            }
            RustdocSource::Root(project_root) => {
                let metadata = manifest_metadata(project_root)?;
                let (selected, skipped) = self.scope.selected_packages(&metadata);
                if selected.is_empty() {
                    let help = if skipped.is_empty() {
                        "".to_string()
                    } else {
                        let skipped = skipped.iter().map(|&p| &p.name).join(", ");
                        format!(
                            "
note: only library targets contain an API surface that can be checked for semver
note: skipped the following crates since they have no library target: {skipped}"
                        )
                    };
                    anyhow::bail!(
                        "no crates with library targets selected, nothing to semver-check{help}"
                    );
                }

                let workspace_overrides =
                    manifest::deserialize_lint_table(&metadata.workspace_metadata)
                        .context("[workspace.metadata.cargo-semver-checks] table is invalid")?
                        .map(Arc::new);

                selected
                    .iter()
                    .map(|selected| {
                        let crate_name = &selected.name;
                        let version = &selected.version;

                        // If the manifest we're using points to a workspace, then
                        // ignore `publish = false` crates unless they are specifically selected.
                        // If the manifest points to a specific crate, then check the crate
                        // even if `publish = false` is set.
                        let is_implied = matches!(self.scope.mode, ScopeMode::DenyList(..))
                            && metadata.workspace_members.len() > 1
                            && selected.publish == Some(vec![]);
                        if is_implied {
                            config.log_verbose(|config| {
                                config.shell_status(
                                    "Skipping",
                                    format_args!("{crate_name} v{version} (current)"),
                                )
                            })?;
                            Ok((crate_name.clone(), None))
                        } else {
                            let package_overrides =
                                manifest::deserialize_lint_table(&selected.metadata)
                                    .with_context(|| {
                                        format!(
                                    "package `{}`'s [package.metadata.cargo-semver-checks] table is invalid (at {})",
                                    selected.name,
                                    selected.manifest_path,
                                )
                                    })?;

                            let mut overrides = OverrideStack::new();
                            if let Some(workspace) = &workspace_overrides {
                                overrides.push(Arc::clone(workspace));
                            }
                            if let Some(package) = package_overrides {
                                overrides.push(Arc::new(package));
                            }

                            let start = std::time::Instant::now();
                            let (current_crate, baseline_crate) = generate_versioned_crates(
                                config,
                                &rustdoc_cmd,
                                &*current_loader,
                                &*baseline_loader,
                                CrateDataForRustdoc {
                                    crate_type: rustdoc_gen::CrateType::Current,
                                    name: crate_name,
                                    feature_config: &self.current_feature_config,
                                    build_target: self.build_target.as_deref(),
                                },
                                CrateDataForRustdoc {
                                    crate_type: rustdoc_gen::CrateType::Baseline {
                                        highest_allowed_version: Some(version),
                                    },
                                    name: crate_name,
                                    feature_config: &self.baseline_feature_config,
                                    build_target: self.build_target.as_deref(),
                                },
                            )?;

                            let result = Ok((
                                crate_name.clone(),
                                Some(run_check_release(
                                    config,
                                    crate_name,
                                    current_crate,
                                    baseline_crate,
                                    self.release_type,
                                    overrides,
                                )?),
                            ));
                            config.shell_status(
                                "Finished",
                                format_args!(
                                    "[{:>8.3}s] {crate_name}",
                                    start.elapsed().as_secs_f32()
                                ),
                            )?;
                            result
                        }
                    })
                    .collect()
            }
        };
        let crate_reports: BTreeMap<String, CrateReport> = {
            let mut reports = BTreeMap::new();
            for outcome in all_outcomes {
                let (name, outcome) = outcome?;
                if let Some(outcome) = outcome {
                    reports.insert(name, outcome);
                }
            }
            reports
        };

        Ok(Report { crate_reports })
    }
}

/// Report of semver check of one crate.
#[non_exhaustive]
#[derive(Debug)]
pub struct CrateReport {
    /// Bump between the current version and the baseline one.
    detected_bump: ActualSemverUpdate,
    /// Minimum additional bump (on top of `detected_bump`) required to respect semver.
    /// For example, if the crate contains breaking changes, this is [`Some(ReleaseType::Major)`].
    /// If no additional bump beyond the already-detected one is required, this is [`Option::None`].
    required_bump: Option<ReleaseType>,
}

impl CrateReport {
    /// Check if the semver check was successful.
    /// `true` if required bump <= detected bump.
    pub fn success(&self) -> bool {
        match self.required_bump {
            // If `None`, no additional bump is required.
            None => true,
            // If `Some`, additional bump is required, so the report is not successful.
            Some(required_bump) => {
                // By design, `required_bump` should always be > `detected_bump`.
                // Let's assert that.
                match self.detected_bump {
                    // If user bumped the major version, any breaking change is accepted.
                    // So `required_bump` should be `None`.
                    ActualSemverUpdate::Major => panic!(
                        "detected_bump is major, while required_bump is {:?}",
                        required_bump
                    ),
                    ActualSemverUpdate::Minor => {
                        assert_eq!(required_bump, ReleaseType::Major);
                    }
                    ActualSemverUpdate::Patch | ActualSemverUpdate::NotChanged => {
                        assert!(matches!(
                            required_bump,
                            ReleaseType::Major | ReleaseType::Minor
                        ));
                    }
                }
                false
            }
        }
    }

    /// Minimum bump required to respect semver.
    /// It's [`Option::None`] if no bump is required beyond the already-detected bump.
    pub fn required_bump(&self) -> Option<ReleaseType> {
        self.required_bump
    }

    /// Bump between the current version and the baseline one.
    pub fn detected_bump(&self) -> ActualSemverUpdate {
        self.detected_bump
    }
}

/// Report of the whole analysis.
/// Contains a report for each crate checked.
#[non_exhaustive]
#[derive(Debug)]
pub struct Report {
    /// Collection containing the name and the report of each crate checked.
    crate_reports: BTreeMap<String, CrateReport>,
}

impl Report {
    /// `true` if none of the crates violate semver.
    pub fn success(&self) -> bool {
        self.crate_reports.values().all(|report| report.success())
    }

    /// Reports of each crate checked, sorted by crate name.
    pub fn crate_reports(&self) -> &BTreeMap<String, CrateReport> {
        &self.crate_reports
    }
}

fn generate_versioned_crates(
    config: &mut GlobalConfig,
    rustdoc_cmd: &RustdocCommand,
    current_loader: &dyn rustdoc_gen::RustdocGenerator,
    baseline_loader: &dyn rustdoc_gen::RustdocGenerator,
    current_crate_data: rustdoc_gen::CrateDataForRustdoc,
    baseline_crate_data: rustdoc_gen::CrateDataForRustdoc,
) -> anyhow::Result<(VersionedCrate, VersionedCrate)> {
    let start = Instant::now();
    let current_path = current_loader.load_rustdoc(config, rustdoc_cmd, current_crate_data)?;
    let current_crate = load_rustdoc(&current_path)?;
    config.shell_status(
        "Parsed",
        format_args!("[{:>8.3}s] (current)", start.elapsed().as_secs_f32()),
    )?;

    let current_rustdoc_version = current_crate.version();

    let start = Instant::now();
    let baseline_path = get_baseline_rustdoc_path(
        config,
        rustdoc_cmd,
        baseline_loader,
        baseline_crate_data.clone(),
    )?;
    let baseline_crate = {
        let mut baseline_crate = load_rustdoc(&baseline_path)?;

        // The baseline rustdoc JSON may have been cached; ensure its rustdoc version matches
        // the version emitted by the currently-installed toolchain.
        //
        // The baseline and current rustdoc JSONs should have the same version.
        // If the baseline rustdoc version doesn't match, delete the cached baseline and rebuild it.
        //
        // Fix for: https://github.com/obi1kenobi/cargo-semver-checks/issues/415
        if baseline_crate.version() != current_rustdoc_version {
            let crate_name = baseline_crate_data.name;
            config.shell_status(
                "Removing",
                format_args!("stale cached baseline rustdoc for {crate_name}"),
            )?;
            std::fs::remove_file(baseline_path)?;
            let baseline_path = get_baseline_rustdoc_path(
                config,
                rustdoc_cmd,
                baseline_loader,
                baseline_crate_data,
            )?;
            baseline_crate = load_rustdoc(&baseline_path)?;

            assert_eq!(
                baseline_crate.version(),
                current_rustdoc_version,
                "Deleting and regenerating the baseline JSON file did not resolve the rustdoc \
                version mismatch."
            );
        }

        baseline_crate
    };
    config.shell_status(
        "Parsed",
        format_args!("[{:>8.3}s] (baseline)", start.elapsed().as_secs_f32()),
    )?;

    Ok((current_crate, baseline_crate))
}

fn get_baseline_rustdoc_path(
    config: &mut GlobalConfig,
    rustdoc_cmd: &RustdocCommand,
    baseline_loader: &dyn rustdoc_gen::RustdocGenerator,
    baseline_crate_data: rustdoc_gen::CrateDataForRustdoc,
) -> anyhow::Result<PathBuf> {
    let baseline_path = baseline_loader.load_rustdoc(config, rustdoc_cmd, baseline_crate_data)?;
    Ok(baseline_path)
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
    std::fs::create_dir_all(cache_dir).context("can't create cache dir")?;
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
