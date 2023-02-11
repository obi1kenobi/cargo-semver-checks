#![forbid(unsafe_code)]

mod check_release;
mod config;
mod manifest;
mod query;
mod rustdoc_cmd;
mod rustdoc_gen;
mod templating;
mod util;

use itertools::Itertools;
use rustdoc_cmd::RustdocCommand;
use semver::Version;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use trustfall_rustdoc::{load_rustdoc, VersionedCrate};

use crate::{
    check_release::run_check_release, config::GlobalConfig, query::ActualSemverUpdate,
    util::slugify,
};

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

            let baseline_loader: Box<dyn rustdoc_gen::RustdocGenerator> =
                if let Some(path) = args.baseline_rustdoc.as_deref() {
                    Box::new(rustdoc_gen::RustdocFromFile::new(path.to_owned()))
                } else if let Some(root) = args.baseline_root.as_deref() {
                    let metadata = args.manifest.metadata().no_deps().exec()?;
                    let target = metadata.target_directory.as_std_path().join(util::SCOPE);
                    Box::new(rustdoc_gen::RustdocFromProjectRoot::new(root, &target)?)
                } else if let Some(rev) = args.baseline_rev.as_deref() {
                    let metadata = args.manifest.metadata().no_deps().exec()?;
                    let source = metadata.workspace_root.as_std_path();
                    let slug = slugify(rev);
                    let target = metadata
                        .target_directory
                        .as_std_path()
                        .join(util::SCOPE)
                        .join(format!("git-{slug}"));
                    Box::new(rustdoc_gen::RustdocFromGitRevision::with_rev(
                        source,
                        &target,
                        rev,
                        &mut config,
                    )?)
                } else {
                    let metadata = args.manifest.metadata().no_deps().exec()?;
                    let target = metadata.target_directory.as_std_path().join(util::SCOPE);
                    let mut registry = rustdoc_gen::RustdocFromRegistry::new(&target, &mut config)?;
                    if let Some(version) = args.baseline_version.as_deref() {
                        let version = semver::Version::parse(version)?;
                        registry.set_version(version);
                    }
                    Box::new(registry)
                };
            let rustdoc_cmd = rustdoc_cmd::RustdocCommand::new()
                .deps(false)
                .silence(!config.is_verbose());

            let all_outcomes: Vec<anyhow::Result<bool>> = if let Some(current_rustdoc_path) =
                args.current_rustdoc.as_deref()
            {
                let name = "<unknown>";
                let baseline_highest_allowed_version = None;
                let current_loader =
                    rustdoc_gen::RustdocFromFile::new(current_rustdoc_path.to_path_buf());
                let (current_crate, baseline_crate) = generate_versioned_crates(
                    &mut config,
                    &rustdoc_cmd,
                    &current_loader,
                    &*baseline_loader,
                    name,
                    baseline_highest_allowed_version,
                )?;

                let success = run_check_release(
                    &mut config,
                    name,
                    current_crate,
                    baseline_crate,
                    args.release_type,
                )?;
                vec![Ok(success)]
            } else {
                let metadata = args.manifest.metadata().exec()?;
                let (selected, _) = args.workspace.partition_packages(&metadata);
                selected
                    .iter()
                    .map(|selected| {
                        let manifest_path = selected.manifest_path.as_std_path();
                        let crate_name = &selected.name;
                        let current_version = &selected.version;

                        let is_implied = args.workspace.all || args.workspace.workspace;
                        if is_implied && selected.publish == Some(vec![]) {
                            config.verbose(|config| {
                                config.shell_status(
                                    "Skipping",
                                    format_args!("{crate_name} v{current_version} (current)"),
                                )
                            })?;
                            Ok(true)
                        } else {
                            let target = metadata.target_directory.as_std_path().join(util::SCOPE);
                            let current_loader = rustdoc_gen::RustdocFromProjectRoot::new(
                                &manifest::get_project_dir_from_manifest_path(manifest_path)?,
                                &target,
                            )?;
                            let (current_crate, baseline_crate) = generate_versioned_crates(
                                &mut config,
                                &rustdoc_cmd,
                                &current_loader,
                                &*baseline_loader,
                                crate_name,
                                Some(current_version),
                            )?;

                            Ok(run_check_release(
                                &mut config,
                                crate_name,
                                current_crate,
                                baseline_crate,
                                args.release_type,
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

fn generate_versioned_crates(
    config: &mut GlobalConfig,
    rustdoc_cmd: &RustdocCommand,
    current_loader: &dyn rustdoc_gen::RustdocGenerator,
    baseline_loader: &dyn rustdoc_gen::RustdocGenerator,
    crate_name: &str,
    version: Option<&Version>,
) -> anyhow::Result<(VersionedCrate, VersionedCrate)> {
    let current_path = current_loader.load_rustdoc(
        config,
        rustdoc_cmd,
        rustdoc_gen::CrateDataForRustdoc {
            name: crate_name,
            crate_type: rustdoc_gen::CrateType::Current,
        },
    )?;
    let current_crate = load_rustdoc(&current_path)?;

    // The process of generating baseline rustdoc can overwrite
    // the already-generated rustdoc of the current crate.
    // For example, this happens when target-dir is specified in `.cargo/config.toml`.
    // That's the reason why we're immediately loading the rustdocs into memory.
    // See: https://github.com/obi1kenobi/cargo-semver-checks/issues/269
    let baseline_path = baseline_loader.load_rustdoc(
        config,
        rustdoc_cmd,
        rustdoc_gen::CrateDataForRustdoc {
            name: crate_name,
            crate_type: rustdoc_gen::CrateType::Baseline {
                highest_allowed_version: version,
            },
        },
    )?;
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

    /// Set the desired release type instead of deriving it from the version number.
    #[arg(
        value_enum,
        long,
        value_name = "TYPE",
        help_heading = "Overrides",
        group = "overrides"
    )]
    release_type: Option<ActualSemverUpdate>,

    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity<clap_verbosity_flag::InfoLevel>,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cargo::command().debug_assert()
}
