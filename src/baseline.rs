use std::collections::BTreeSet;

use anyhow::Context as _;
use crates_index::Crate;

use crate::dump::RustDocCommand;
use crate::util::slugify;
use crate::GlobalConfig;

pub(crate) trait BaselineLoader {
    fn load_rustdoc(
        &self,
        config: &mut GlobalConfig,
        rustdoc: &RustDocCommand,
        name: &str,
        version_current: Option<&semver::Version>,
    ) -> anyhow::Result<std::path::PathBuf>;
}

pub(crate) struct RustdocBaseline {
    path: std::path::PathBuf,
}

impl RustdocBaseline {
    pub(crate) fn new(path: std::path::PathBuf) -> Self {
        Self { path }
    }
}

impl BaselineLoader for RustdocBaseline {
    fn load_rustdoc(
        &self,
        _config: &mut GlobalConfig,
        _rustdoc: &RustDocCommand,
        _name: &str,
        _version_current: Option<&semver::Version>,
    ) -> anyhow::Result<std::path::PathBuf> {
        Ok(self.path.clone())
    }
}

pub(crate) struct PathBaseline {
    root: std::path::PathBuf,
    lookup: std::collections::HashMap<String, (String, std::path::PathBuf)>,
}

impl PathBaseline {
    pub(crate) fn new(root: &std::path::Path) -> anyhow::Result<Self> {
        let mut lookup = std::collections::HashMap::new();
        for result in ignore::Walk::new(root) {
            let entry = result?;
            if entry.file_name() == "Cargo.toml" {
                if let Ok(manifest) = crate::manifest::Manifest::parse(entry.path()) {
                    if let (Ok(name), Ok(version)) = (
                        crate::manifest::get_package_name(&manifest),
                        crate::manifest::get_package_version(&manifest),
                    ) {
                        lookup.insert(name, (version, entry.into_path()));
                    }
                }
            }
        }
        Ok(Self {
            root: root.to_owned(),
            lookup,
        })
    }
}

impl BaselineLoader for PathBaseline {
    fn load_rustdoc(
        &self,
        config: &mut GlobalConfig,
        rustdoc: &RustDocCommand,
        name: &str,
        _version_current: Option<&semver::Version>,
    ) -> anyhow::Result<std::path::PathBuf> {
        let (version, manifest_path) = self
            .lookup
            .get(name)
            .with_context(|| format!("package `{}` not found in {}", name, self.root.display()))?;
        let version = format!(" v{version}");
        config.shell_status("Parsing", format_args!("{name}{version} (baseline)"))?;
        let rustdoc_path = rustdoc.dump(manifest_path.as_path(), None, true)?;
        Ok(rustdoc_path)
    }
}

pub(crate) struct GitBaseline {
    path: PathBaseline,
}

impl GitBaseline {
    pub fn with_rev(
        source: &std::path::Path,
        target: &std::path::Path,
        rev: &str,
        config: &mut GlobalConfig,
    ) -> anyhow::Result<Self> {
        config.shell_status("Cloning", rev)?;
        let repo = git2::Repository::discover(source)?;

        let rev = repo.revparse_single(rev)?;
        let target = target.join(rev.id().to_string());

        std::fs::create_dir_all(&target)?;
        let tree = rev.peel_to_tree()?;
        extract_tree(&repo, tree, &target)?;

        let path = PathBaseline::new(&target)?;
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

impl BaselineLoader for GitBaseline {
    fn load_rustdoc(
        &self,
        config: &mut GlobalConfig,
        rustdoc: &RustDocCommand,
        name: &str,
        version_current: Option<&semver::Version>,
    ) -> anyhow::Result<std::path::PathBuf> {
        self.path
            .load_rustdoc(config, rustdoc, name, version_current)
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

pub(crate) struct RegistryBaseline {
    target_root: std::path::PathBuf,
    version: Option<semver::Version>,
    index: crates_index::Index,
}

impl RegistryBaseline {
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

/// To get the rustdoc of the baseline, we first create a placeholder project somewhere
/// with the baseline as a dependency, and run `cargo rustdoc` on it.
fn create_rustdoc_manifest_for_crate_version(
    crate_baseline: &crates_index::Version,
) -> cargo_toml::Manifest<()> {
    use cargo_toml::*;

    Manifest::<()> {
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
            // Sometimes crates ship types with fields or variants that are included
            // only when certain features are enabled.
            //
            // The current crate's rustdoc is generated with `--all-features`.
            // We need the baseline to be generated with all features too,
            // but that option isn't available here so we have to implement it ourselves.
            //
            // Fixes:
            // - regular features: https://github.com/obi1kenobi/cargo-semver-check/issues/147
            // - implicit features from optional dependencies:
            //     https://github.com/obi1kenobi/cargo-semver-checks/issues/265
            let mut implicit_features: BTreeSet<_> = crate_baseline
                .dependencies()
                .iter()
                .filter_map(|dep| dep.is_optional().then_some(dep.name()))
                .map(|x| x.to_string())
                .collect();
            for feature_defn in crate_baseline.features().values().flatten() {
                // "If you specify the optional dependency with the dep: prefix anywhere
                //  in the [features] table, that disables the implicit feature."
                // https://doc.rust-lang.org/cargo/reference/features.html#optional-dependencies
                if let Some(optional_dep) = feature_defn.strip_prefix("dep:") {
                    implicit_features.remove(optional_dep);
                }
            }
            let regular_features: BTreeSet<_> = crate_baseline.features().keys().cloned().collect();
            let mut all_features = implicit_features;
            all_features.extend(regular_features);

            let project_with_features = DependencyDetail {
                // We need the *exact* version as a dependency, or else cargo will
                // give us the latest semver-compatible version which is not we want.
                // Fixes: https://github.com/obi1kenobi/cargo-semver-checks/issues/261
                version: Some(format!("={}", crate_baseline.version())),
                features: all_features.into_iter().collect(),
                ..DependencyDetail::default()
            };
            let mut deps = DepsSet::new();
            deps.insert(
                crate_baseline.name().to_string(),
                Dependency::Detailed(project_with_features),
            );
            deps
        },
        ..Default::default()
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

impl BaselineLoader for RegistryBaseline {
    fn load_rustdoc(
        &self,
        config: &mut GlobalConfig,
        rustdoc: &RustDocCommand,
        name: &str,
        version_current: Option<&semver::Version>,
    ) -> anyhow::Result<std::path::PathBuf> {
        let crate_ = self
            .index
            .crate_(name)
            .with_context(|| anyhow::format_err!("{} not found in registry", name))?;

        let base_version = if let Some(base) = self.version.as_ref() {
            base.to_string()
        } else {
            choose_baseline_version(&crate_, version_current)?
        };

        let base_root = self.target_root.join(format!(
            "registry-{}-{}",
            slugify(name),
            slugify(&base_version)
        ));
        std::fs::create_dir_all(&base_root)?;
        let manifest_path = base_root.join("Cargo.toml");

        let crate_baseline = crate_
            .versions()
            .iter()
            .find(|v| v.version() == base_version)
            .with_context(|| {
                anyhow::format_err!(
                    "Version {} of crate {} not found in registry",
                    name,
                    base_version
                )
            })?;

        // Possibly fixes https://github.com/libp2p/rust-libp2p/pull/2647#issuecomment-1280221217
        let _: std::io::Result<()> = std::fs::remove_file(base_root.join("Cargo.lock"));

        std::fs::write(
            &manifest_path,
            toml::to_string(&create_rustdoc_manifest_for_crate_version(crate_baseline))?,
        )?;
        std::fs::write(base_root.join("lib.rs"), "")?;

        config.shell_status("Parsing", format_args!("{name} v{base_version} (baseline)"))?;
        let rustdoc_path = rustdoc.dump(
            manifest_path.as_path(),
            Some(&format!("{name}@{base_version}")),
            false,
        )?;
        Ok(rustdoc_path)
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
