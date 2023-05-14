use std::path::PathBuf;

use anyhow::Context;
use crates_index::Crate;
use itertools::Itertools;
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::manifest::Manifest;
use crate::rustdoc_cmd::RustdocCommand;
use crate::util::slugify;
use crate::GlobalConfig;

#[derive(Debug, Clone)]
pub(crate) enum CrateSource<'a> {
    Registry { crate_: &'a crates_index::Version },
    ManifestPath { manifest: &'a Manifest },
}

impl<'a> CrateSource<'a> {
    pub(crate) fn name(&self) -> anyhow::Result<&str> {
        Ok(match self {
            Self::Registry { crate_ } => crate_.name(),
            Self::ManifestPath { manifest } => crate::manifest::get_package_name(manifest)?,
        })
    }

    pub(crate) fn version(&self) -> anyhow::Result<&str> {
        Ok(match self {
            Self::Registry { crate_ } => crate_.version(),
            Self::ManifestPath { manifest } => crate::manifest::get_package_version(manifest)?,
        })
    }

    /// A path-safe unique identifier that includes the crate's name, version, and source.
    pub(crate) fn slug(&self) -> anyhow::Result<String> {
        Ok(format!(
            "{}-{}-{}",
            match self {
                CrateSource::Registry { .. } => "registry",
                CrateSource::ManifestPath { .. } => "local",
            },
            slugify(self.name()?),
            slugify(self.version()?)
        ))
    }

    /// Returns features listed in `[features]` section in the manifest
    /// <https://doc.rust-lang.org/cargo/reference/features.html#the-features-section>
    pub(crate) fn regular_features(&self) -> Vec<String> {
        match self {
            Self::Registry { crate_ } => crate_.features().keys().cloned().collect(),
            Self::ManifestPath { manifest } => manifest.parsed.features.keys().cloned().collect(),
        }
    }

    /// Returns features implicitly defined by optional dependencies
    /// <https://doc.rust-lang.org/cargo/reference/features.html#optional-dependencies>
    pub(crate) fn implicit_features(&self) -> std::collections::BTreeSet<String> {
        let mut implicit_features: std::collections::BTreeSet<_> = match self {
            Self::Registry { crate_ } => crate_
                .dependencies()
                .iter()
                .filter_map(|dep| dep.is_optional().then_some(dep.name()))
                .map(|x| x.to_string())
                .collect(),
            Self::ManifestPath { manifest } => {
                let mut dependencies = manifest.parsed.dependencies.clone();
                for target in manifest.parsed.target.values() {
                    // Fixes https://github.com/obi1kenobi/cargo-semver-checks/issues/369
                    // This part is not relevant to `Self::Registry`, because
                    // it doesn't have a `target` field and doesn't differentiate dependencies
                    // between different targets.
                    dependencies.extend(target.dependencies.clone().into_iter());
                }
                dependencies
                    .iter()
                    .filter_map(|(name, dep)| dep.optional().then_some(name))
                    .map(|x| x.to_string())
                    .collect()
            }
        };

        let feature_defns: Vec<&String> = match self {
            Self::Registry { crate_ } => crate_.features().values().flatten().collect(),
            Self::ManifestPath { manifest } => {
                manifest.parsed.features.values().flatten().collect()
            }
        };

        for feature_defn in feature_defns {
            // "If you specify the optional dependency with the dep: prefix anywhere
            //  in the [features] table, that disables the implicit feature."
            // https://doc.rust-lang.org/cargo/reference/features.html#optional-dependencies
            if let Some(optional_dep) = feature_defn.strip_prefix("dep:") {
                implicit_features.remove(optional_dep);
            }
        }
        implicit_features
    }

    /// Sometimes crates ship types with fields or variants that are included
    /// only when certain features are enabled.
    ///
    /// By default, we want to generate rustdoc with `--all-features`,
    /// but that option isn't available outside of the current crate,
    /// so we have to implement it ourselves.
    pub(crate) fn all_features(&self) -> Vec<String> {
        // Implicit features from optional dependencies have to be added separately
        // from regular features: https://github.com/obi1kenobi/cargo-semver-checks/issues/265
        let mut all_crate_features = self.implicit_features();
        all_crate_features.extend(self.regular_features());
        all_crate_features.into_iter().collect()
    }

    /// Sometimes crates include features that are not meant for public use
    /// or otherwise don't adhere to semver:
    ///  - private features, like `bench` used for internal benchmarking only
    ///  - nightly-only features, like `nightly`
    ///  - unstable features containing experimental code, like `unstable`
    ///
    /// To ensure the best possible out-of-the-box user experience,
    /// this function attempts to heuristically exclude feature names like above.
    ///
    /// The heuristics are based on the name since cargo does not currently include
    /// a mechanism for marking features as private/hidden/unstable. When such
    /// mechanisms are available in cargo, we'll update this functionality to make
    /// use of them. Relevant cargo issues:
    /// - unstable/nightly-only features: <https://github.com/rust-lang/cargo/issues/10881>
    /// - private/hidden features:        <https://github.com/rust-lang/cargo/issues/10882>
    ///
    /// Because of the above, this function filters out features with names:
    ///  - `unstable`
    ///  - `nightly`
    ///  - `bench`
    ///  - `no_std`
    ///  - features with prefix `__`
    fn heuristically_included_features(&self) -> Vec<String> {
        let features_ignored_by_default = std::collections::HashSet::from([
            String::from("unstable"),
            String::from("nightly"),
            String::from("bench"),
            String::from("no_std"),
        ]);

        let determine = |feature_name: &String| {
            !features_ignored_by_default.contains(feature_name) && !feature_name.starts_with("__")
        };

        self.all_features().into_iter().filter(determine).collect()
    }

    /// Returns features to explicitly enable. Does not fetch default features,
    /// which are enabled separately.
    ///
    /// For baseline version, the extra features that do not exist are ignored,
    /// because they could be just added to the current version.
    /// A warning is issued in this case.
    pub(crate) fn feature_list_from_config(
        &self,
        global_config: &mut GlobalConfig,
        feature_config: &FeatureConfig,
    ) -> Vec<String> {
        let all_features: std::collections::HashSet<String> =
            self.all_features().into_iter().collect();

        let result = [
            match feature_config.features_group {
                FeaturesGroup::All => self.all_features(),
                FeaturesGroup::Heuristic => self.heuristically_included_features(),
                FeaturesGroup::Default | FeaturesGroup::None => vec![],
            },
            feature_config.extra_features.clone(),
        ]
        .concat();

        result
            .into_iter()
            .filter(|feature_name| {
                if !all_features.contains(feature_name) && feature_config.is_baseline {
                    global_config
                        .shell_warn(format!(
                            "Feature `{feature_name}` is not present in the baseline."
                        ))
                        .expect("print failed");
                    false
                } else {
                    true
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum CrateType<'a> {
    Current,
    Baseline {
        /// When the baseline is being generated from registry
        /// and no specific version was chosen, we want to select a version
        /// that is the same or older than the version of the current crate.
        highest_allowed_version: Option<&'a semver::Version>,
    },
}

/// Configuration used to choose features to enable.
/// Separate configs are used for baseline and current versions.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct FeatureConfig {
    /// Feature set chosen as the foundation.
    pub(crate) features_group: FeaturesGroup,
    /// Explicitly enabled features.
    pub(crate) extra_features: Vec<String>,
    pub(crate) is_baseline: bool,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) enum FeaturesGroup {
    All,
    Default,
    Heuristic,
    None,
}

impl FeatureConfig {
    pub(crate) fn default_for_current() -> Self {
        // The default behaviour for both version is the heuristic approach.
        Self {
            features_group: FeaturesGroup::Heuristic,
            extra_features: Vec::new(),
            is_baseline: false,
        }
    }

    pub(crate) fn default_for_baseline() -> Self {
        Self {
            features_group: FeaturesGroup::Heuristic,
            extra_features: Vec::new(),
            is_baseline: true,
        }
    }

    /// Unique identifier, used to mark generated rustdoc files.
    /// `is_baseline` is ignored there, as rustdoc is cached only for baseline.
    fn make_identifier(&self) -> String {
        if matches!(self.features_group, FeaturesGroup::Heuristic) && self.extra_features.is_empty()
        {
            // If using the default config, return empty identifier
            return String::new();
        }

        // Sort the features, because the order is not revelant.
        let mut to_serialize = self.clone();
        to_serialize.extra_features.sort();

        // Serialize the whole struct. `The is_baseline` field is also serialized,
        // as it changes the behaviour of the check. It has no negative effect
        // on cache hitting, as only baseline crates are cached.
        let serialized = serde_json::to_string(&to_serialize).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(serialized.as_bytes());

        // Store the hash as string with hex number (leading zeros added)
        let mut hash = format!("{:0>64x}", hasher.finalize());

        // First 16 characters is good enough for our use case.
        // For birthday paradox to occur, single crate version must be run
        // with an order of 2**32 feature configurations.
        hash.truncate(16);
        hash
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CrateDataForRustdoc<'a> {
    pub(crate) crate_type: CrateType<'a>,
    pub(crate) name: &'a str,
    pub(crate) feature_config: &'a FeatureConfig,
}

impl<'a> CrateType<'a> {
    pub(crate) fn type_name(&self) -> &'static str {
        match self {
            CrateType::Current => "current",
            CrateType::Baseline { .. } => "baseline",
        }
    }
}

fn generate_rustdoc(
    config: &mut GlobalConfig,
    rustdoc_cmd: &RustdocCommand,
    target_root: PathBuf,
    crate_source: CrateSource,
    crate_data: CrateDataForRustdoc,
) -> anyhow::Result<PathBuf> {
    let name = crate_source.name()?;
    let version = crate_source.version()?;
    let feature_identifier = crate_data.feature_config.make_identifier();
    let crate_identifier = if feature_identifier.is_empty() {
        crate_source.slug()?
    } else {
        format!(
            "{}-{}",
            crate_source.slug()?,
            crate_data.feature_config.make_identifier(),
        )
    };

    let (cache_dir, cached_rustdoc) = match crate_source {
        CrateSource::Registry { .. } => {
            let cache_dir = target_root.join("cache");
            let cached_rustdoc = cache_dir.join(format!("{crate_identifier}.json"));

            // We assume that the generated rustdoc is untouched.
            // Users should run cargo-clean if they experience any anomalies.
            if cached_rustdoc.exists() {
                config.shell_status(
                    "Parsing",
                    format_args!(
                        "{name} v{version} ({}, cached)",
                        crate_data.crate_type.type_name()
                    ),
                )?;
                return Ok(cached_rustdoc);
            }

            (Some(cache_dir), Some(cached_rustdoc))
        }
        CrateSource::ManifestPath { .. } => {
            // Manifest-based crates cannot be cached since they correspond
            // to a specific (and unknown) gitrev and git state which is not part of their slug.
            // There's no way to check whether a cached entry is a match or not.
            (None, None)
        }
    };

    config.shell_status(
        "Parsing",
        format_args!("{name} v{version} ({})", crate_data.crate_type.type_name()),
    )?;

    let build_dir = target_root.join(&crate_identifier);
    let rustdoc_path =
        rustdoc_cmd.generate_rustdoc(config, build_dir.clone(), &crate_source, &crate_data)?;

    match crate_source {
        CrateSource::Registry { .. } => {
            // Clean up after ourselves.
            let cache_dir = cache_dir.expect(
                "when crate_source is Registry a cache_dir was created, so it should be Some",
            );
            let cached_rustdoc = cached_rustdoc.expect(
                "when crate_source is Registry a cached_rustdoc was created, so it should be Some",
            );
            std::fs::create_dir_all(cache_dir)?;
            std::fs::copy(rustdoc_path, &cached_rustdoc)?;
            std::fs::remove_dir_all(build_dir)?;

            Ok(cached_rustdoc)
        }
        CrateSource::ManifestPath { .. } => {
            // We don't do any caching here -- since the crate is saved locally,
            // it could be modified by the user after it was cached.
            Ok(rustdoc_path)
        }
    }
}

pub(crate) trait RustdocGenerator {
    fn load_rustdoc(
        &self,
        config: &mut GlobalConfig,
        rustdoc_cmd: &RustdocCommand,
        crate_data: CrateDataForRustdoc,
    ) -> anyhow::Result<PathBuf>;
}

#[derive(Debug)]
pub(crate) struct RustdocFromFile {
    path: PathBuf,
}

impl RustdocFromFile {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl RustdocGenerator for RustdocFromFile {
    fn load_rustdoc(
        &self,
        _config: &mut GlobalConfig,
        _rustdoc_cmd: &RustdocCommand,
        _crate_data: CrateDataForRustdoc,
    ) -> anyhow::Result<PathBuf> {
        Ok(self.path.clone())
    }
}

#[derive(Debug)]
pub(crate) struct RustdocFromProjectRoot {
    project_root: PathBuf,
    manifests: std::collections::HashMap<String, Manifest>,
    manifest_errors: std::collections::HashMap<PathBuf, anyhow::Error>,
    target_root: PathBuf,
}

impl RustdocFromProjectRoot {
    /// # Arguments
    /// * `project_root` - Path to a directory with the manifest or with subdirectories with the manifests.
    /// * `target_root` - Path to a directory where the placeholder manifest / rustdoc can be created.
    pub(crate) fn new(
        project_root: &std::path::Path,
        target_root: &std::path::Path,
    ) -> anyhow::Result<Self> {
        let mut manifests = std::collections::HashMap::new();
        let mut manifest_errors = std::collections::HashMap::new();
        for result in ignore::Walk::new(project_root) {
            let entry = result?;
            if entry.file_name() == "Cargo.toml" {
                let path = entry.into_path();
                match crate::manifest::Manifest::parse(path.clone()) {
                    Ok(manifest) => match crate::manifest::get_package_name(&manifest) {
                        Ok(name) => {
                            manifests.insert(name.to_string(), manifest);
                        }
                        Err(e) => {
                            manifest_errors.insert(path, e);
                        }
                    },
                    Err(e) => {
                        manifest_errors.insert(path, e);
                    }
                }
            }
        }
        Ok(Self {
            project_root: project_root.to_owned(),
            manifests,
            manifest_errors,
            target_root: target_root.to_owned(),
        })
    }
}

impl RustdocGenerator for RustdocFromProjectRoot {
    fn load_rustdoc(
        &self,
        config: &mut GlobalConfig,
        rustdoc_cmd: &RustdocCommand,
        crate_data: CrateDataForRustdoc,
    ) -> anyhow::Result<PathBuf> {
        let manifest: &Manifest = self.manifests.get(crate_data.name).ok_or_else(|| {
            let err = anyhow::anyhow!(
                "package `{}` not found in {}",
                crate_data.name,
                self.project_root.display(),
            );
            if self.manifest_errors.is_empty() {
                err
            } else {
                let cause_list = self
                    .manifest_errors
                    .values()
                    .map(|error| format!("  {error:#},"))
                    .join("\n");
                let possible_causes = format!("possibly due to errors: [\n{cause_list}\n]");
                err.context(possible_causes)
            }
        })?;
        generate_rustdoc(
            config,
            rustdoc_cmd,
            self.target_root.clone(),
            CrateSource::ManifestPath { manifest },
            crate_data,
        )
    }
}

#[derive(Debug)]
pub(crate) struct RustdocFromGitRevision {
    path: RustdocFromProjectRoot,
}

impl RustdocFromGitRevision {
    pub fn with_rev(
        source: &std::path::Path,
        target: &std::path::Path,
        rev: &str,
        config: &mut GlobalConfig,
    ) -> anyhow::Result<Self> {
        config.shell_status("Cloning", rev)?;
        let repo = git2::Repository::discover(source)?;

        let rev = repo.revparse_single(rev)?;
        let rev_dir = target.join(rev.id().to_string());

        std::fs::create_dir_all(&rev_dir)?;
        let tree = rev.peel_to_tree()?;
        extract_tree(&repo, tree, &rev_dir)?;

        let path = RustdocFromProjectRoot::new(&rev_dir, target)?;
        Ok(Self { path })
    }
}

fn extract_tree(
    repo: &git2::Repository,
    tree: git2::Tree<'_>,
    target: &std::path::Path,
) -> anyhow::Result<()> {
    for entry in tree.iter() {
        match entry.kind() {
            Some(git2::ObjectType::Tree) => {
                let object = entry.to_object(repo);
                let tree = object.and_then(|o| o.peel_to_tree());
                if let Ok(tree) = tree {
                    let path = target.join(bytes2str(entry.name_bytes()));
                    std::fs::create_dir_all(&path)?;
                    extract_tree(repo, tree, &path)?;
                }
            }
            Some(git2::ObjectType::Blob) => {
                let object = entry.to_object(repo);
                let blob = object.and_then(|o| o.peel_to_blob());
                if let Ok(blob) = blob {
                    let path = target.join(bytes2str(entry.name_bytes()));
                    let existing = std::fs::read(&path).ok();
                    if existing.as_deref() != Some(blob.content()) {
                        std::fs::write(&path, blob.content())?;
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}

impl RustdocGenerator for RustdocFromGitRevision {
    fn load_rustdoc(
        &self,
        config: &mut GlobalConfig,
        rustdoc_cmd: &RustdocCommand,
        crate_data: CrateDataForRustdoc,
    ) -> anyhow::Result<PathBuf> {
        self.path.load_rustdoc(config, rustdoc_cmd, crate_data)
    }
}

// From git2 crate
#[cfg(unix)]
fn bytes2str(b: &[u8]) -> &std::ffi::OsStr {
    use std::os::unix::prelude::*;
    std::ffi::OsStr::from_bytes(b)
}

// From git2 crate
#[cfg(windows)]
fn bytes2str(b: &[u8]) -> &std::ffi::OsStr {
    use std::str;
    std::ffi::OsStr::new(str::from_utf8(b).unwrap())
}

pub(crate) struct RustdocFromRegistry {
    target_root: PathBuf,
    version: Option<semver::Version>,
    index: crates_index::Index,
}

impl core::fmt::Debug for RustdocFromRegistry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RustdocFromRegistry")
            .field("target_root", &self.target_root)
            .field("version", &self.version)
            .field("index", &"<elided>")
            .finish()
    }
}

impl RustdocFromRegistry {
    pub fn new(target_root: &std::path::Path, config: &mut GlobalConfig) -> anyhow::Result<Self> {
        let mut index = crates_index::Index::new_cargo_default()?;

        config.shell_status("Updating", "index")?;
        while need_retry(index.update())? {
            config.shell_status("Blocking", "waiting for lock on registry index")?;
            std::thread::sleep(REGISTRY_BACKOFF);
        }

        Ok(Self {
            target_root: target_root.to_owned(),
            version: None,
            index,
        })
    }

    pub fn set_version(&mut self, version: semver::Version) {
        self.version = Some(version);
    }
}

fn choose_baseline_version(
    crate_: &Crate,
    version_current: Option<&semver::Version>,
) -> anyhow::Result<String> {
    // Try to avoid pre-releases
    // - Breaking changes are allowed between them
    // - Most likely the user cares about the last official release
    if let Some(current) = version_current {
        let mut instances = crate_
            .versions()
            .iter()
            .filter_map(|i| {
                semver::Version::parse(i.version())
                    .map(|v| (v, i.is_yanked()))
                    .ok()
            })
            // For unpublished changes when the user doesn't increment the version
            // post-release, allow using the current version as a baseline.
            .filter(|(v, _)| v <= current)
            .collect::<Vec<_>>();
        instances.sort();
        instances
            .iter()
            .rev()
            .find(|(v, yanked)| v.pre.is_empty() && !yanked)
            .or_else(|| instances.last())
            .map(|(v, _)| v.to_string())
            .with_context(|| {
                anyhow::format_err!(
                    "No available baseline versions for {}@{}",
                    crate_.name(),
                    current
                )
            })
    } else {
        let instance = crate_
            .highest_normal_version()
            .unwrap_or_else(|| {
                // If there is no normal version (not yanked and not a pre-release)
                // choosing the latest one anyway is more reasonable than throwing an
                // error, as there is still a chance that it is what the user expects.
                crate_.highest_version()
            })
            .version();
        Ok(instance.to_owned())
    }
}

impl RustdocGenerator for RustdocFromRegistry {
    fn load_rustdoc(
        &self,
        config: &mut GlobalConfig,
        rustdoc_cmd: &RustdocCommand,
        crate_data: CrateDataForRustdoc,
    ) -> anyhow::Result<PathBuf> {
        let crate_ = self
            .index
            .crate_(crate_data.name)
            .with_context(|| anyhow::format_err!("{} not found in registry", crate_data.name))?;

        let base_version = if let Some(base) = self.version.as_ref() {
            base.to_string()
        } else {
            choose_baseline_version(
                &crate_,
                match crate_data.crate_type {
                    CrateType::Current => None,
                    CrateType::Baseline {
                        highest_allowed_version,
                    } => highest_allowed_version,
                },
            )?
        };

        let crate_ = crate_
            .versions()
            .iter()
            .find(|v| v.version() == base_version)
            .with_context(|| {
                anyhow::format_err!(
                    "Version {} of crate {} not found in registry",
                    crate_data.name,
                    base_version
                )
            })?;

        generate_rustdoc(
            config,
            rustdoc_cmd,
            self.target_root.clone(),
            CrateSource::Registry { crate_ },
            crate_data,
        )
    }
}

const REGISTRY_BACKOFF: std::time::Duration = std::time::Duration::from_secs(1);

/// Check if we need to retry retrieving the Index.
fn need_retry(res: Result<(), crates_index::Error>) -> anyhow::Result<bool> {
    match res {
        Ok(()) => Ok(false),
        Err(crates_index::Error::Git(err)) => {
            if err.class() == git2::ErrorClass::Index && err.code() == git2::ErrorCode::Locked {
                Ok(true)
            } else {
                Err(crates_index::Error::Git(err).into())
            }
        }
        Err(err) => Err(err.into()),
    }
}

#[cfg(test)]
mod tests {
    use crates_index::{Crate, Version};

    use super::choose_baseline_version;

    fn new_mock_version(version_name: &str, yanked: bool) -> Version {
        // `crates_index::Version` cannot be created explicitly, as all its fields
        // are private, so we use the fact that it can be deserialized.
        serde_json::from_value(serde_json::json!({
            "name": "test-crate",
            "vers": version_name,
            "deps": [],
            "features": {},
            "yanked": yanked,
            "cksum": "00".repeat(32),
        }))
        .expect("hand-written JSON used to create mock crates_index::Version should be valid")
    }

    fn new_crate(versions: Vec<Version>) -> Crate {
        // `crates_index::Crate` cannot be created explicitly, as its field
        // is private, so we use the fact that it can be deserialized.
        serde_json::from_value(serde_json::json!({ "versions": versions }))
            .expect("hand-written JSON used to create mock crates_index::Crate should be valid")
    }

    fn assert_correctly_picks_baseline_version(
        versions: Vec<(&str, bool)>,
        current_version_name: Option<&str>,
        expected: &str,
    ) {
        let crate_ = new_crate(
            versions
                .into_iter()
                .map(|(version, yanked)| new_mock_version(version, yanked))
                .collect(),
        );
        let current_version = current_version_name.map(|version_name| {
            semver::Version::parse(version_name)
                .expect("current_version_name used in assertion should encode a valid version")
        });
        let chosen_baseline = choose_baseline_version(&crate_, current_version.as_ref())
            .expect("choose_baseline_version should not return any error in the test case");
        assert_eq!(chosen_baseline, expected.to_owned());
    }

    #[test]
    fn baseline_choosing_logic_skips_yanked() {
        assert_correctly_picks_baseline_version(
            vec![("1.2.0", false), ("1.2.1", true)],
            Some("1.2.2"),
            "1.2.0",
        );
    }

    #[test]
    fn baseline_choosing_logic_skips_greater_than_current() {
        assert_correctly_picks_baseline_version(
            vec![("1.2.0", false), ("1.2.1", false)],
            Some("1.2.0"),
            "1.2.0",
        );
    }

    #[test]
    fn baseline_choosing_logic_skips_pre_releases() {
        assert_correctly_picks_baseline_version(
            vec![("1.2.0", false), ("1.2.1-rc1", false)],
            Some("1.2.1-rc2"),
            "1.2.0",
        );
    }

    #[test]
    fn baseline_choosing_logic_without_current_picks_latest_normal() {
        assert_correctly_picks_baseline_version(
            vec![("1.2.0", false), ("1.2.1-rc1", false), ("1.3.1", true)],
            None,
            "1.2.0",
        );
    }

    #[test]
    fn baseline_choosing_logic_picks_pre_release_if_there_is_no_normal() {
        assert_correctly_picks_baseline_version(
            vec![("1.2.0", true), ("1.2.1-rc1", false)],
            Some("1.2.1"),
            "1.2.1-rc1",
        );
    }

    #[test]
    fn baseline_choosing_logic_picks_yanked_if_there_is_no_normal() {
        assert_correctly_picks_baseline_version(
            vec![("1.2.1-rc1", false), ("1.2.1", true)],
            Some("1.2.1"),
            "1.2.1",
        );
    }
}
