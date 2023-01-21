#![forbid(unsafe_code)]

mod baseline;
mod check_release;
mod config;
mod dump;
mod manifest;
mod query;
mod templating;
mod util;

use dump::RustDocCommand;
use itertools::Itertools;
use semver::Version;
use std::path::{Path, PathBuf};

use clap::{Args, Parser, Subcommand};
use trustfall_rustdoc::{load_rustdoc, VersionedCrate};

use crate::{check_release::run_check_release, config::GlobalConfig, util::slugify};

fn main() -> anyhow::Result<()> {
    human_panic::setup_panic!();

    let Cargo::SemverChecks(args) = Cargo::parse();
    if args.bugreport {
        use bugreport::{bugreport, collector::*, format::Markdown};
        bugreport!()
            .info(SoftwareVersion::default())
            .info(OperatingSystem::default())
            .info(CommandLine::default())
            .info(CommandOutput::new("cargo version", "cargo", &["-V"]))
            .info(CompileTimeInformation::default())
            .print::<Markdown>();
        std::process::exit(0);
    } else if args.list {
        let queries = query::SemverQuery::all_queries();
        let mut rows = vec![["id", "type", "description"], ["==", "====", "==========="]];
        for query in queries.values() {
            rows.push([
                query.id.as_str(),
                query.required_update.as_str(),
                query.description.as_str(),
            ]);
        }
        let mut widths = [0; 3];
        for row in &rows {
            widths[0] = widths[0].max(row[0].len());
            widths[1] = widths[1].max(row[1].len());
            widths[2] = widths[2].max(row[2].len());
        }
        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();
        for row in rows {
            use std::io::Write;
            writeln!(
                stdout,
                "{0:<1$} {2:<3$} {4:<5$}",
                row[0], widths[0], row[1], widths[1], row[2], widths[2]
            )?;
        }

        let mut config = GlobalConfig::new().set_level(args.verbosity.log_level());
        config.shell_note("Use `--explain <id>` to see more details")?;
        std::process::exit(0);
    } else if let Some(id) = args.explain.as_deref() {
        let queries = query::SemverQuery::all_queries();
        let query = queries.get(id).ok_or_else(|| {
            let ids = queries.keys().cloned().collect::<Vec<_>>();
            anyhow::format_err!(
                "Unknown id `{}`, available id's:\n  {}",
                id,
                ids.join("\n  ")
            )
        })?;
        println!(
            "{}",
            query
                .reference
                .as_deref()
                .unwrap_or(query.description.as_str())
        );
        if let Some(link) = &query.reference_link {
            println!();
            println!("See also {link}");
        }
        std::process::exit(0);
    }

    match args.command {
        Some(SemverChecksCommands::CheckRelease(args)) => {
            let mut config = GlobalConfig::new().set_level(args.verbosity.log_level());

            let loader: Box<dyn baseline::BaselineLoader> =
                if let Some(path) = args.baseline_rustdoc.as_deref() {
                    Box::new(baseline::RustdocBaseline::new(path.to_owned()))
                } else if let Some(root) = args.baseline_root.as_deref() {
                    Box::new(baseline::PathBaseline::new(root)?)
                } else if let Some(rev) = args.baseline_rev.as_deref() {
                    let metadata = args.manifest.metadata().no_deps().exec()?;
                    let source = metadata.workspace_root.as_std_path();
                    let slug = slugify(rev);
                    let target = metadata
                        .target_directory
                        .as_std_path()
                        .join(util::SCOPE)
                        .join(format!("git-{slug}"));
                    Box::new(baseline::GitBaseline::with_rev(
                        source,
                        &target,
                        rev,
                        &mut config,
                    )?)
                } else {
                    let metadata = args.manifest.metadata().no_deps().exec()?;
                    let target = metadata.target_directory.as_std_path().join(util::SCOPE);
                    let mut registry = baseline::RegistryBaseline::new(&target, &mut config)?;
                    if let Some(version) = args.baseline_version.as_deref() {
                        let version = semver::Version::parse(version)?;
                        registry.set_version(version);
                    }
                    Box::new(registry)
                };
            let rustdoc_cmd = dump::RustDocCommand::new()
                .deps(false)
                .silence(!config.is_verbose());

            let all_outcomes: Vec<anyhow::Result<bool>> = if let Some(current_rustdoc_path) =
                args.current_rustdoc.as_deref()
            {
                let name = "<unknown>";
                let (current_crate, baseline_crate) = generate_versioned_crates(
                    &mut config,
                    CurrentCratePath::CurrentRustdocPath(current_rustdoc_path),
                    &loader,
                    &rustdoc_cmd,
                    name,
                    None,
                )?;

                let success = run_check_release(&mut config, name, current_crate, baseline_crate)?;
                vec![Ok(success)]
            } else {
                let metadata = args.manifest.metadata().exec()?;
                let (selected, _) = args.workspace.partition_packages(&metadata);
                selected
                    .iter()
                    .map(|selected| {
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
                            Ok(true)
                        } else {
                            config.shell_status(
                                "Parsing",
                                format_args!("{crate_name} v{version} (current)"),
                            )?;

                            let (current_crate, baseline_crate) = generate_versioned_crates(
                                &mut config,
                                CurrentCratePath::ManifestPath(manifest_path),
                                &loader,
                                &rustdoc_cmd,
                                crate_name,
                                Some(version),
                            )?;

                            Ok(run_check_release(
                                &mut config,
                                crate_name,
                                current_crate,
                                baseline_crate,
                            )?)
                        }
                    })
                    .collect()
            };

            let success = all_outcomes
                .into_iter()
                .fold_ok(true, std::ops::BitAnd::bitand)?;
            if success {
                std::process::exit(0);
            } else {
                std::process::exit(1);
            }
        }
        None => {
            anyhow::bail!("subcommand required");
        }
    }
}

// Argument to the generate_versioned_crates function.
// It can be either a path to an existing rustdoc, or a manifest path of a crate.
enum CurrentCratePath<'a> {
    CurrentRustdocPath(&'a Path),
    ManifestPath(&'a Path),
}

fn generate_versioned_crates(
    config: &mut GlobalConfig,
    current_crate_path: CurrentCratePath,
    loader: &Box<dyn baseline::BaselineLoader>,
    rustdoc_cmd: &RustDocCommand,
    crate_name: &str,
    version: Option<&Version>,
) -> anyhow::Result<(VersionedCrate, VersionedCrate)> {
    let current_crate = match current_crate_path {
        CurrentCratePath::CurrentRustdocPath(rustdoc_path) => load_rustdoc(rustdoc_path)?,
        CurrentCratePath::ManifestPath(manifest_path) => {
            let rustdoc_path = rustdoc_cmd.dump(manifest_path, None, true)?;
            load_rustdoc(&rustdoc_path)?
        }
    };

    // The process of generating baseline rustdoc can overwrite
    // the already-generated rustdoc of the current crate.
    // For example, this happens when target-dir is specified in `.cargo/config.toml`.
    // That's the reason why we're immediately loading the rustdocs into memory.
    // See: https://github.com/obi1kenobi/cargo-semver-checks/issues/269
    let baseline_path = loader.load_rustdoc(config, &rustdoc_cmd, crate_name, version)?;
    let baseline_crate = load_rustdoc(&baseline_path)?;

    Ok((current_crate, baseline_crate))
}

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
#[command(version, propagate_version = true)]
enum Cargo {
    SemverChecks(SemverChecks),
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
struct SemverChecks {
    #[arg(long, global = true, exclusive = true)]
    bugreport: bool,

    #[arg(long, global = true, exclusive = true)]
    explain: Option<String>,

    #[arg(long, global = true, exclusive = true)]
    list: bool,

    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity<clap_verbosity_flag::InfoLevel>,

    #[command(subcommand)]
    command: Option<SemverChecksCommands>,
}

/// Check your crate for semver violations.
#[derive(Subcommand)]
enum SemverChecksCommands {
    #[command(alias = "diff-files")]
    CheckRelease(CheckRelease),
}

#[derive(Args)]
struct CheckRelease {
    #[command(flatten, next_help_heading = "Current")]
    pub manifest: clap_cargo::Manifest,

    #[command(flatten, next_help_heading = "Current")]
    pub workspace: clap_cargo::Workspace,

    /// The current rustdoc json output to test for semver violations.
    #[arg(
        long,
        short_alias = 'c',
        alias = "current",
        value_name = "JSON_PATH",
        help_heading = "Current",
        requires = "baseline_rustdoc"
    )]
    current_rustdoc: Option<PathBuf>,

    /// Version from registry to lookup for a baseline
    #[arg(
        long,
        value_name = "X.Y.Z",
        help_heading = "Baseline",
        group = "baseline"
    )]
    baseline_version: Option<String>,

    /// Git revision to lookup for a baseline
    #[arg(
        long,
        value_name = "REV",
        help_heading = "Baseline",
        group = "baseline"
    )]
    baseline_rev: Option<String>,

    /// Directory containing baseline crate source
    #[arg(
        long,
        value_name = "MANIFEST_ROOT",
        help_heading = "Baseline",
        group = "baseline"
    )]
    baseline_root: Option<PathBuf>,

    /// The rustdoc json file to use as a semver baseline.
    #[arg(
        long,
        short_alias = 'b',
        alias = "baseline",
        value_name = "JSON_PATH",
        help_heading = "Baseline",
        group = "baseline"
    )]
    baseline_rustdoc: Option<PathBuf>,

    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity<clap_verbosity_flag::InfoLevel>,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cargo::command().debug_assert()
}
