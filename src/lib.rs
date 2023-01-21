mod baseline;
mod check_release;
mod config;
mod dump;
mod manifest;
mod query;
mod templating;
mod util;

use config::GlobalConfig;
pub use dump::*;
pub use templating::*;

use std::path::PathBuf;

/// Test a release for semver violations.
#[derive(Default)]
pub struct Check {
    /// Which packages to analyze.
    scope: Scope,
    current: Current,
    baseline: Baseline,
    log_level: Option<log::Level>,
}

#[derive(Default)]
enum Baseline {
    /// Version from registry to lookup for a baseline. E.g. "1.0.0".
    Version(String),
    /// Git revision to lookup for a baseline.
    Revision(String),
    /// Directory containing baseline crate source.
    Root(PathBuf),
    /// The rustdoc json file to use as a semver baseline.
    RustDoc(PathBuf),
    /// Latest version published to the cargo registry.
    #[default]
    LatestVersion,
}

/// Current version of the project to analyze.
#[derive(Default)]
enum Current {
    /// Path to the manifest of the current version of the project.
    /// It can be a workspace or a single package.
    Manifest(PathBuf),
    /// The rustdoc json of the current version of the project.
    RustDoc(PathBuf),
    /// Use the manifest in the current directory.
    #[default]
    CurrentDir,
}

/// Which packages to analyze.
#[derive(Default)]
enum Scope {
    /// A subset of packages in the workspace.
    Packages(Vec<String>),
    /// All packages in the workspace.
    #[default]
    Workspace,
    /// All packages in the workspace, except these.
    Exclude(Vec<String>),
}

impl Check {
    pub fn new() -> Self {
        Self::default()
    }

    fn manifest_path(&self) -> anyhow::Result<PathBuf> {
        let path = match &self.current {
            Current::Manifest(path) => path.clone(),
            Current::RustDoc(_) => {
                anyhow::bail!("error: RustDoc is not supported with these arguments.")
            }
            Current::CurrentDir => PathBuf::from("Cargo.toml"),
        };
        Ok(path)
    }

    fn manifest_metadata_no_deps(&self) -> anyhow::Result<cargo_metadata::Metadata> {
        let mut command = cargo_metadata::MetadataCommand::new();
        let metadata = command
            .manifest_path(self.manifest_path()?)
            .no_deps()
            .exec()?;
        Ok(metadata)
    }

    pub fn check_release(&self) -> anyhow::Result<Report> {
        let mut config = GlobalConfig::new().set_level(self.log_level);

        let loader: Box<dyn baseline::BaselineLoader> = match self.baseline {
            Baseline::Version(version) => {
                let mut registry = self.registry_baseline(&mut config)?;
                let version = semver::Version::parse(&version)?;
                registry.set_version(version);
                Box::new(registry)
            }
            Baseline::Revision(rev) => {
                let metadata = self.manifest_metadata_no_deps()?;
                let source = metadata.workspace_root.as_std_path();
                let slug = util::slugify(&rev);
                let target = metadata
                    .target_directory
                    .as_std_path()
                    .join(util::SCOPE)
                    .join(format!("git-{slug}"));
                Box::new(baseline::GitBaseline::with_rev(
                    source,
                    &target,
                    &rev,
                    &mut config,
                )?)
            }
            Baseline::Root(root) => Box::new(baseline::PathBaseline::new(&root)?),
            Baseline::RustDoc(path) => Box::new(baseline::RustdocBaseline::new(path.to_owned())),
            Baseline::LatestVersion => {
                let metadata = self.manifest_metadata_no_deps()?;
                let target = metadata.target_directory.as_std_path().join(util::SCOPE);
                let mut registry = baseline::RegistryBaseline::new(&target, &mut config)?;
                Box::new(registry)
            }
        };
        let rustdoc_cmd = dump::RustDocCommand::new()
            .deps(false)
            .silence(!config.is_verbose());

            let rustdoc_paths = match self.current {
                Current::Manifest(_) => todo!(),
                Current::RustDoc(rustdoc_path) => {

            let name = "<unknown>";
            let version = None;
            vec![(
                name.to_owned(),
                loader.load_rustdoc(&mut config, &rustdoc_cmd, name, version)?,
                rustdoc_path.to_owned(),
            )]
                }
                Current::CurrentDir => todo!(),
            }
        let rustdoc_paths =
         {
            let metadata = args.manifest.metadata().exec()?;
            let (selected, _) = args.workspace.partition_packages(&metadata);
            let mut rustdoc_paths = Vec::with_capacity(selected.len());
            for selected in selected {
                let manifest_path = selected.manifest_path.as_std_path();
                let crate_name = &selected.name;
                let version = &selected.version;

                let is_implied = args.workspace.all || args.workspace.workspace;
                if is_implied && selected.publish == Some(vec![]) {
                    config.verbose(|config| {
                        config.shell_status(
                            "Skipping",
                            format_args!("{crate_name} v{version} (current)"),
                        )
                    })?;
                    continue;
                }

                config
                    .shell_status("Parsing", format_args!("{crate_name} v{version} (current)"))?;
                let rustdoc_path = rustdoc_cmd.dump(manifest_path, None, true)?;
                let baseline_path =
                    loader.load_rustdoc(&mut config, &rustdoc_cmd, crate_name, Some(version))?;
                rustdoc_paths.push((crate_name.clone(), baseline_path, rustdoc_path));
            }
            rustdoc_paths
        };
        let mut success = true;
        for (crate_name, baseline_path, current_path) in rustdoc_paths {
            let baseline_crate = load_rustdoc(&baseline_path)?;
            let current_crate = load_rustdoc(&current_path)?;

            if !run_check_release(&mut config, &crate_name, current_crate, baseline_crate)? {
                success = false;
            }
        }

        Ok(Report {})
    }

    fn registry_baseline(&self, config: &mut GlobalConfig) -> Result<baseline::RegistryBaseline, anyhow::Error> {
        let metadata = self.manifest_metadata_no_deps()?;
        let target = metadata.target_directory.as_std_path().join(util::SCOPE);
        let mut registry = baseline::RegistryBaseline::new(&target, config)?;
        Ok(registry)
    }
}

pub struct Report {}
