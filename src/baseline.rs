use anyhow::Context as _;

use crate::dump::RustDocCommand;
use crate::util::slugify;
use crate::GlobalConfig;

pub(crate) trait BaselineLoader {
    fn load_rustdoc(
        &self,
        config: &mut GlobalConfig,
        rustdoc: &RustDocCommand,
        name: &str,
        version: Option<&semver::Version>,
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
        _version: Option<&semver::Version>,
    ) -> anyhow::Result<std::path::PathBuf> {
        Ok(self.path.clone())
    }
}

pub(crate) struct PathBaseline {
    root: std::path::PathBuf,
    lookup: std::collections::HashMap<String, std::path::PathBuf>,
}

impl PathBaseline {
    pub(crate) fn new(root: &std::path::Path) -> anyhow::Result<Self> {
        let mut lookup = std::collections::HashMap::new();
        for result in ignore::Walk::new(root) {
            let entry = result?;
            if entry.file_name() == "Cargo.toml" {
                if let Ok(name) = crate::manifest::Manifest::parse(entry.path())
                    .and_then(|manifest| crate::manifest::get_package_name(&manifest))
                {
                    lookup.insert(name, entry.into_path());
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
        version: Option<&semver::Version>,
    ) -> anyhow::Result<std::path::PathBuf> {
        let manifest_path = self
            .lookup
            .get(name)
            .with_context(|| format!("package `{}` not found in {}", name, self.root.display()))?;
        let version = version.map(|v| format!(" v{}", v)).unwrap_or_default();
        config.shell_status("Parsing", format_args!("{}{} (baseline)", name, version))?;
        let rustdoc_path = rustdoc.dump(manifest_path.as_path(), None)?;
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
        version: Option<&semver::Version>,
    ) -> anyhow::Result<std::path::PathBuf> {
        self.path.load_rustdoc(config, rustdoc, name, version)
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

    // To get the rustdoc of the baseline, we first somewhere create a dummy project
    // with the baseline as a dependency and on in we launch `cargo rustdoc`.
    fn create_manifest_for_rustdoc(
        &self,
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
                let project_with_features = DependencyDetail {
                    version: Some(crate_baseline.version().to_string()),
                    // adding features fixes:
                    // https://github.com/obi1kenobi/cargo-semver-check/issues/147
                    features: crate_baseline
                        .features()
                        .iter()
                        .map(|(key, _values)| key.clone())
                        .collect(),
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
}

impl BaselineLoader for RegistryBaseline {
    fn load_rustdoc(
        &self,
        config: &mut GlobalConfig,
        rustdoc: &RustDocCommand,
        name: &str,
        version: Option<&semver::Version>,
    ) -> anyhow::Result<std::path::PathBuf> {
        let crate_ = self
            .index
            .crate_(name)
            .with_context(|| anyhow::format_err!("{} not found in registry", name))?;
        // Try to avoid pre-releases
        // - Breaking changes are allowed between them
        // - Most likely the user cares about the last official release
        let base_version = if let Some(base) = self.version.as_ref() {
            base.to_string()
        } else if let Some(current) = version {
            let mut instances = crate_
                .versions()
                .iter()
                .filter_map(|i| semver::Version::parse(i.version()).ok())
                // For unpublished changes when the user doesn't increment the version
                // post-release, allow using the current version as a baseline.
                .filter(|v| v <= current)
                .collect::<Vec<_>>();
            instances.sort();
            let instance = instances
                .iter()
                .rev()
                .find(|v| v.pre.is_empty())
                .or_else(|| instances.last())
                .with_context(|| {
                    anyhow::format_err!("No available baseline versions for {}@{}", name, current)
                })?;
            instance.to_string()
        } else {
            let instance = crate_
                .highest_normal_version()
                .unwrap_or_else(|| crate_.highest_version());
            instance.version().to_owned()
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

        std::fs::write(
            &manifest_path,
            toml::to_string(&self.create_manifest_for_rustdoc(crate_baseline))?
                .replace("edition = \"2021\"", ""),
        )?;
        std::fs::write(base_root.join("lib.rs"), "")?;

        config.shell_status(
            "Parsing",
            format_args!("{} v{} (baseline)", name, base_version),
        )?;
        let rustdoc_path = rustdoc.dump(
            manifest_path.as_path(),
            Some(&format!("{}@{}", name, base_version)),
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
