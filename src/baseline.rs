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

        std::fs::create_dir_all(target)?;
        let mut co = git2::build::CheckoutBuilder::new();
        co.target_dir(target)
            .remove_untracked(true)
            .remove_ignored(true)
            .use_ours(true)
            .force();
        repo.checkout_tree(&rev, Some(&mut co))?;

        let path = PathBaseline::new(target)?;
        Ok(Self { path })
    }
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
