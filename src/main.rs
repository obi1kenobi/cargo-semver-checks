#![forbid(unsafe_code)]

pub mod adapter;
mod baseline;
mod check_release;
mod dump;
pub mod indexed_crate;
mod manifest;
mod query;
mod templating;
mod util;

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use termcolor::{ColorChoice, StandardStream};

use crate::baseline::BaselineLoader;
use crate::{
    check_release::run_check_release, templating::make_handlebars_registry,
    util::load_rustdoc_from_file,
};

#[allow(dead_code)]
pub(crate) struct GlobalConfig {
    printing_to_terminal: bool,
    output_writer: StandardStream,
    handlebars: handlebars::Handlebars<'static>,
}

impl GlobalConfig {
    fn new() -> Self {
        let printing_to_terminal = atty::is(atty::Stream::Stdout);

        let color_choice = match std::env::var("CARGO_TERM_COLOR").as_deref() {
            Ok("always") => ColorChoice::Always,
            Ok("alwaysansi") => ColorChoice::AlwaysAnsi,
            Ok("auto") => ColorChoice::Auto,
            Ok("never") => ColorChoice::Never,
            Ok(_) | Err(..) => {
                if printing_to_terminal {
                    ColorChoice::Auto
                } else {
                    ColorChoice::Never
                }
            }
        };

        Self {
            printing_to_terminal,
            output_writer: StandardStream::stdout(color_choice),
            handlebars: make_handlebars_registry(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let Cargo::SemverChecks(args) = Cargo::parse();

    let mut config = GlobalConfig::new();

    match args {
        SemverChecks::CheckRelease(args) => {
            let loader = Box::new(baseline::RustdocBaseline::new(
                args.baseline_rustdoc_path.clone(),
            ));
            let rustdoc = dump::RustDoc::new().deps(false).silence(false);

            let rustdoc_paths =
                if let Some(current_rustdoc_path) = args.current_rustdoc_path.as_deref() {
                    vec![(
                        loader.load_rustdoc("<unknown>")?,
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
                        let rustdoc_path = rustdoc.dump(manifest_path)?;
                        let crate_name = manifest::get_package_name(manifest_path)?;
                        let baseline_path = loader.load_rustdoc(&crate_name)?;
                        rustdoc_paths.push((baseline_path, rustdoc_path));
                    }
                    rustdoc_paths
                };
            let mut success = true;
            for (baseline_path, current_path) in rustdoc_paths {
                let baseline_crate = load_rustdoc_from_file(&baseline_path)?;
                let current_crate = load_rustdoc_from_file(&current_path)?;

                if !run_check_release(&mut config, current_crate, baseline_crate)? {
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
struct CheckRelease {
    #[clap(flatten)]
    pub manifest: clap_cargo::Manifest,

    #[clap(flatten)]
    pub workspace: clap_cargo::Workspace,

    /// The current rustdoc json output to test for semver violations.
    #[clap(
        short,
        long = "current",
        value_name = "CURRENT_RUSTDOC_JSON",
        requires = "baseline-rustdoc-path"
    )]
    current_rustdoc_path: Option<PathBuf>,

    /// The rustdoc json file to use as a semver baseline.
    #[clap(short, long = "baseline", value_name = "BASELINE_RUSTDOC_JSON")]
    baseline_rustdoc_path: PathBuf,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cargo::command().debug_assert()
}
