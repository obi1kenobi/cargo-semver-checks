use anyhow::Context as _;

use crate::dump::RustDocCommand;
use crate::GlobalConfig;

pub(crate) trait BaselineLoader {
    fn load_rustdoc(
        &self,
        config: &mut GlobalConfig,
        rustdoc: &RustDocCommand,
        name: &str,
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
                if let Ok(name) = crate::manifest::get_package_name(entry.path()) {
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
    ) -> anyhow::Result<std::path::PathBuf> {
        let manifest_path = self
            .lookup
            .get(name)
            .with_context(|| format!("package `{}` not found in {}", name, self.root.display()))?;
        config.shell_status("Parsing", format_args!("{} (baseline)", name))?;
        let rustdoc_path = rustdoc.dump(manifest_path.as_path())?;
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
                    if existing.as_ref().map(|v| v.as_slice()) != Some(blob.content()) {
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
    ) -> anyhow::Result<std::path::PathBuf> {
        self.path.load_rustdoc(config, rustdoc, name)
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
