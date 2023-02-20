use std::path::{Path, PathBuf};

use anyhow::Context;
use crates_index::Crate;

use crate::manifest::Manifest;
use crate::rustdoc_cmd::RustdocCommand;
use crate::util::slugify;
use crate::GlobalConfig;

#[derive(Debug, Clone)]
enum CrateSource<'a> {
    Registry { crate_: &'a crates_index::Version },
    ManifestPath { manifest: &'a Manifest },
}

impl<'a> CrateSource<'a> {
    fn name(&self) -> anyhow::Result<&str> {
        Ok(match self {
            Self::Registry { crate_ } => crate_.name(),
            Self::ManifestPath { manifest } => crate::manifest::get_package_name(manifest)?,
        })
    }

    fn version(&self) -> anyhow::Result<&str> {
        Ok(match self {
            Self::Registry { crate_ } => crate_.version(),
            Self::ManifestPath { manifest } => crate::manifest::get_package_version(manifest)?,
        })
    }

    /// Returns features listed in `[features]` section in the manifest
    /// <https://doc.rust-lang.org/cargo/reference/features.html#the-features-section>
    fn regular_features(&self) -> Vec<String> {
        match self {
            Self::Registry { crate_ } => crate_.features().keys().cloned().collect(),
            Self::ManifestPath { manifest } => manifest.parsed.features.keys().cloned().collect(),
        }
    }

    /// Returns features implicitly defined by optional dependencies
    /// <https://doc.rust-lang.org/cargo/reference/features.html#optional-dependencies>
    fn implicit_features(&self) -> std::collections::BTreeSet<String> {
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
    fn all_features(&self) -> Vec<String> {
        // Implicit features from optional dependencies have to be added separately
        // from regular features: https://github.com/obi1kenobi/cargo-semver-checks/issues/265
        let mut all_crate_features = self.implicit_features();
        all_crate_features.extend(self.regular_features());
        all_crate_features.into_iter().collect()
    }
}

/// To get the rustdoc of the project, we first create a placeholder project somewhere
/// with the project as a dependency, and run `cargo rustdoc` on it.
fn create_placeholder_rustdoc_manifest(
    crate_source: &CrateSource,
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

#[derive(Debug, Clone)]
pub(crate) struct CrateDataForRustdoc<'a> {
    pub(crate) crate_type: CrateType<'a>,
    pub(crate) name: &'a str,
    // TODO: pass an enum describing which features to enable
}

impl<'a> CrateType<'a> {
    fn type_name(&self) -> &'static str {
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

    let crate_identifier = format!(
        "{}-{}-{}",
        match crate_source {
            CrateSource::Registry { .. } => "registry",
            // Identifiers of manifest-based crates cannot be used for caching,
            // since they probably correspond to a specific (and unknown) gitrev and git state
            // so cached entries cannot be checked to see if they are a match or not.
            CrateSource::ManifestPath { .. } => "local",
        },
        slugify(name),
        slugify(version)
    );
    let build_dir = target_root.join(&crate_identifier);
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
        CrateSource::ManifestPath { .. } => (None, None),
    };

    let placeholder_manifest = create_placeholder_rustdoc_manifest(&crate_source)
        .context("failed to create placeholder manifest")?;
    let placeholder_manifest_path =
        save_placeholder_rustdoc_manifest(build_dir.as_path(), placeholder_manifest)
            .context("failed to save placeholder rustdoc manifest")?;

    config.shell_status(
        "Parsing",
        format_args!("{name} v{version} ({})", crate_data.crate_type.type_name()),
    )?;

    let rustdoc_path = rustdoc_cmd.dump(
        placeholder_manifest_path.as_path(),
        Some(&format!("{name}@{version}")),
        false,
    )?;

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
            // it could be modified by the user since it was cached.
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
    lookup: std::collections::HashMap<String, Manifest>,
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
        let mut lookup = std::collections::HashMap::new();
        for result in ignore::Walk::new(project_root) {
            let entry = result?;
            if entry.file_name() == "Cargo.toml" {
                if let Ok(manifest) = crate::manifest::Manifest::parse(entry.into_path()) {
                    if let Ok(name) = crate::manifest::get_package_name(&manifest) {
                        lookup.insert(name.to_string(), manifest);
                    }
                }
            }
        }
        Ok(Self {
            project_root: project_root.to_owned(),
            lookup,
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
        let manifest: &Manifest = self.lookup.get(crate_data.name).with_context(|| {
            format!(
                "package `{}` not found in {}",
                crate_data.name,
                self.project_root.display()
            )
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
