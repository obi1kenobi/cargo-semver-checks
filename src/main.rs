#![forbid(unsafe_code)]

pub mod adapter;
mod baseline;
mod check_release;
mod config;
mod dump;
pub mod indexed_crate;
mod manifest;
mod query;
mod templating;
mod util;

use std::path::PathBuf;

use clap::{ArgGroup, Args, Parser, Subcommand};

use crate::{check_release::run_check_release, util::load_rustdoc_from_file};
use config::GlobalConfig;

fn main() -> anyhow::Result<()> {
    let Cargo::SemverChecks(args) = Cargo::parse();

    match args {
        SemverChecks::CheckRelease(args) => {
            let mut config = GlobalConfig::new().set_level(args.verbosity.log_level());

            let loader: Box<dyn baseline::BaselineLoader> =
                if let Some(path) = args.baseline_rustdoc.as_deref() {
                    Box::new(baseline::RustdocBaseline::new(path.to_owned()))
                } else if let Some(root) = args.baseline_root.as_deref() {
                    Box::new(baseline::PathBaseline::new(root)?)
                } else {
                    unreachable!("a member of the `baseline` group must be present");
                };
            let rustdoc_cmd = dump::RustDocCommand::new()
                .deps(false)
                .silence(!config.is_verbose());

            let rustdoc_paths = if let Some(current_rustdoc_path) = args.current_rustdoc.as_deref()
            {
                let name = "<unknown>";
                vec![(
                    name.to_owned(),
                    loader.load_rustdoc(&mut config, &rustdoc_cmd, name)?,
                    current_rustdoc_path.to_owned(),
                )]
            } else {
                let metadata = args.manifest.metadata().exec()?;
                let (selected, _) = args.workspace.partition_packages(&metadata);
                if selected.len() != 1 {
                    anyhow::bail!(
                        "only one package can be processed at a time: {}",
                        selected
                            .into_iter()
                            .map(|s| s.name.clone())
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }
                let mut rustdoc_paths = Vec::with_capacity(selected.len());
                for selected in selected {
                    let manifest_path = selected.manifest_path.as_std_path();
                    let crate_name = manifest::get_package_name(manifest_path)?;
                    config.shell_status("Parsing", format_args!("{} (current)", crate_name))?;
                    let rustdoc_path = rustdoc_cmd.dump(manifest_path)?;
                    let baseline_path =
                        loader.load_rustdoc(&mut config, &rustdoc_cmd, &crate_name)?;
                    rustdoc_paths.push((crate_name, baseline_path, rustdoc_path));
                }
                rustdoc_paths
            };
            let mut success = true;
            for (crate_name, baseline_path, current_path) in rustdoc_paths {
                let baseline_crate = load_rustdoc_from_file(&baseline_path)?;
                let current_crate = load_rustdoc_from_file(&current_path)?;

                if !run_check_release(&mut config, &crate_name, current_crate, baseline_crate)? {
                    success = false;
                }
            }
            if success {
                std::process::exit(0);
            } else {
                std::process::exit(1);
            }
        }
    }
}

#[derive(Parser)]
#[clap(name = "cargo")]
#[clap(bin_name = "cargo")]
#[clap(version, propagate_version = true)]
enum Cargo {
    #[clap(subcommand)]
    SemverChecks(SemverChecks),
}

/// Check your crate for semver violations.
#[derive(Subcommand)]
enum SemverChecks {
    #[clap(alias = "diff-files")]
    CheckRelease(CheckRelease),
}

#[derive(Args)]
#[clap(group = ArgGroup::new("baseline").required(true))]
#[clap(setting = clap::AppSettings::DeriveDisplayOrder)]
struct CheckRelease {
    #[clap(flatten, next_help_heading = "CURRENT")]
    pub manifest: clap_cargo::Manifest,

    #[clap(flatten, next_help_heading = "CURRENT")]
    pub workspace: clap_cargo::Workspace,

    /// The current rustdoc json output to test for semver violations.
    #[clap(
        long,
        short_alias = 'c',
        alias = "current",
        value_name = "JSON_PATH",
        help_heading = "CURRENT",
        requires = "baseline-rustdoc"
    )]
    current_rustdoc: Option<PathBuf>,

    /// Directory containing baseline crate source
    #[clap(
        long,
        value_name = "MANIFEST_ROOT",
        help_heading = "BASELINE",
        group = "baseline"
    )]
    baseline_root: Option<PathBuf>,

    /// The rustdoc json file to use as a semver baseline.
    #[clap(
        long,
        short_alias = 'b',
        alias = "baseline",
        value_name = "JSON_PATH",
        help_heading = "BASELINE",
        group = "baseline"
    )]
    baseline_rustdoc: Option<PathBuf>,

    #[clap(flatten)]
    verbosity: clap_verbosity_flag::Verbosity<clap_verbosity_flag::InfoLevel>,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cargo::command().debug_assert()
}
