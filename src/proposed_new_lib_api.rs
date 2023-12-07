#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckGroup {
    /// All the checks we should run.
    checks: Vec<CrateCheck>,
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrateCheck {
    /// The version we are interested in semver-checking.
    current_version: CrateVersion,

    /// The version we are comparing the current version against.
    baseline_version: CrateVersion,

    /// Optionally, override what kind of release this is and therefore which lints we check.
    ///
    /// If `None`, we'll determine the appropriate lints based on the version numbers
    /// of the current and baseline crate versions.
    assume_release_type: Option<ReleaseType>
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrateVersion {
    /// The name of the crate we're checking.
    crate_name: String,

    /// Where are we getting the source code for this crate version from?
    source: Source,

    /// Which group of features to enable: all features, default features, no features,
    /// or use our own heuristic that aims to select all intended-to-be-public features.
    ///
    /// TODO: move `FeaturesGroup` out of `rustdoc_gen.rs` and make it `pub` and `#[non_exhaustive]`
    features_group: FeaturesGroup,

    /// Additional features to enable, on top of the ones selected by `features_group`.
    extra_features: Vec<String>,

    /// What to do if the specified version has no library target, only binary targets.
    ///
    /// In this situation, there's no API to semver-check since this version
    /// cannot be used as a lib dependency, regardless of which APIs are declared `pub`.
    if_package_has_no_library_target: OnUnexpectedOutcome,

    /// What to do if a requested feature does not exist.
    if_feature_does_not_exist: OnUnexpectedOutcome,

    /// What to do if we failed to generate or parse rustdoc.
    ///
    /// For example, if the package fails to compile with the current Rust version and rustflags.
    if_failed_to_get_rustdoc: OnUnexpectedOutcome,
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    /// Load the crate's data from local source code.
    ManifestPath(ManifestPath),

    /// Load the crate's data from a specific git revision in a local repo.
    ManifestGitRevision(ManifestGitRevision),

    /// Load the crate's data from a specific version on crates.io.
    ExactRegistryVersion(ExactRegistryVersion),

    /// Load the crate's data by selecting a version from crates.io based on specific requirements.
    PredicateRegistryVersion(PredicateRegistryVersion),
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestPath {
    /// The `Cargo.toml` of the workspace containing the crate to check,
    /// or the crate's own `Cargo.toml` if the crate isn't part of a workspace.
    root_manifest: std::path::PathBuf,

    /// If we failed to open, read, or parse the root manifest file at the specified path.
    if_failed_to_read_root_manifest: OnUnexpectedOutcome,

    /// If we opened the manifest but did not find the specified package inside it.
    if_failed_to_find_package: OnUnexpectedOutcome,
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestGitRevision {
    /// Path to the repository's root directory.
    repo: std::path::PathBuf,

    /// The git revision we should check out and use for semver-checking.
    rev: String,

    /// Path (relative to the repo root) to the `Cargo.toml` of the workspace
    /// containing the crate to check, or to the crate's own `Cargo.toml`
    /// if the crate isn't part of a workspace.
    root_manifest: std::path::PathBuf,

    /// If we failed to use the git repo and check out the revision.
    /// For example, if the repo path is incorrect or the specified revision doesn't exist.
    if_failed_to_check_out_rev: OnUnexpectedOutcome

    /// If we failed to open, read, or parse the root manifest file at the specified path.
    if_failed_to_read_root_manifest: OnUnexpectedOutcome,

    /// If we opened the manifest but did not find the specified package inside it.
    /// For example, perhaps the package does not yet exist at the specified revision.
    if_failed_to_find_package: OnUnexpectedOutcome,
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactRegistryVersion {
    /// The crate version to check.
    version: semver::Version,

    /// If we aren't able to use the registry, for example due to a connection error.
    if_failed_to_use_registry: OnUnexpectedOutcome,

    /// If we connected to the registry but could not find the specified package.
    if_failed_to_find_package: OnUnexpectedOutcome,

    /// If the registry knows about the package, but does not have the specified version.
    if_failed_to_find_package_version: OnUnexpectedOutcome,

    /// If the registry has the specified version but it is not usable for semver-checking:
    /// for example, if it has been yanked (we won't be able to generate rustdoc for it).
    if_version_is_unusable: OnUnexpectedOutcome,
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredicateRegistryVersion {
    /// The way to select the version.
    predicate: VersionPredicate,

    /// Whether to consider pre-release versions or not.
    ///
    /// Note that breaking changes are always allowed between pre-releases,
    /// and between a pre-release and a corresponding normal release.
    allow_prereleases: bool,

    /// Whether we should automatically ignore yanked releases.
    ignore_yanked: bool,

    /// If we aren't able to use the registry, for example due to a connection error.
    if_failed_to_use_registry: OnUnexpectedOutcome,

    /// If we connected to the registry but could not find the specified package.
    if_failed_to_find_package: OnUnexpectedOutcome,

    /// If the registry knows about the package, but none of its versions match our requirements.
    if_no_matching_version: OnUnexpectedOutcome,

    /// If a version matched our requirements, but it is not usable for semver-checking:
    /// for example, if it has been yanked (we won't be able to generate rustdoc for it).
    if_version_is_unusable: OnUnexpectedOutcome,
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionPredicate {
    /// Strictly less than the specified version.
    LessThan(semver::Version),

    /// Less than or equal than the specified version.
    LessThanOrEqual(semver::Version),

    /// The largest version number available.
    HighestAvailable,

    /// The last release ordered by release date.
    MostRecentlyPublished.
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OnUnexpectedOutcome {
    /// Stop running checks and return an error.
    FailStop,

    /// Log an error but continue with other checks.
    LogAndContinue,

    /// Continue with other checks as if nothing happened.
    /// Do not print anything to stderr.
    ContinueSilently,
}
