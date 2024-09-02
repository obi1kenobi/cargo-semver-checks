#![forbid(unsafe_code)]

use std::{env, path::PathBuf};

use anstyle::{AnsiColor, Color, Reset, Style};
use cargo_config2::Config;
use cargo_semver_checks::{
    GlobalConfig, PackageSelection, ReleaseType, Rustdoc, ScopeSelection, SemverQuery,
};
use clap::{Args, Parser, Subcommand};
use std::io::Write;

#[cfg(test)]
mod snapshot_tests;

fn main() {
    human_panic::setup_panic!();

    let Cargo::SemverChecks(args) = Cargo::parse();

    configure_color(args.color_choice);
    let mut config = GlobalConfig::new();
    config.set_log_level(args.verbosity.log_level());

    if args.bugreport {
        print_issue_url(&mut config);
        std::process::exit(0);
    } else if args.list {
        exit_on_error(true, || {
            let queries = SemverQuery::all_queries();
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

            config.shell_note("Use `--explain <id>` to see more details")
        });
        std::process::exit(0);
    } else if let Some(id) = args.explain.as_deref() {
        exit_on_error(true, || {
            let queries = SemverQuery::all_queries();
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
            Ok(())
        });
        std::process::exit(0);
    }

    let check_release = match args.command {
        Some(SemverChecksCommands::CheckRelease(c)) => c,
        None => args.check_release,
    };

    let check: cargo_semver_checks::Check = check_release.into();

    let report = exit_on_error(config.is_error(), || check.check_release(&mut config));
    if report.success() {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}

fn exit_on_error<T>(log_errors: bool, mut inner: impl FnMut() -> anyhow::Result<T>) -> T {
    match inner() {
        Ok(x) => x,
        Err(err) => {
            if log_errors {
                eprintln!("error: {err:?}");
            }
            std::process::exit(1)
        }
    }
}

/// helper function to determine whether to use colors based on the (passed) `--color` flag
/// and the value of the `CARGO_TERM_COLOR` variable.
///
/// If the `--color` flag is set to something valid, it overrides anything in
/// the `CARGO_TERM_COLOR` environment variable
fn configure_color(cli_choice: Option<clap::ColorChoice>) {
    use anstream::ColorChoice as AnstreamChoice;
    use clap::ColorChoice as ClapChoice;
    let choice = match cli_choice {
        Some(ClapChoice::Always) => AnstreamChoice::Always,
        Some(ClapChoice::Auto) => AnstreamChoice::Auto,
        Some(ClapChoice::Never) => AnstreamChoice::Never,
        // we match the behavior of cargo in
        // https://doc.rust-lang.org/cargo/reference/config.html#termcolor
        // note that [`ColorChoice::AlwaysAnsi`] is not supported by cargo.
        None => match env::var("CARGO_TERM_COLOR").as_deref() {
            Ok("always") => AnstreamChoice::Always,
            Ok("never") => AnstreamChoice::Never,
            // if `auto` is set, or the env var is invalid
            // or both the env var and flag are not set, we set the choice to auto
            _ => AnstreamChoice::Auto,
        },
    };

    choice.write_global();
}

fn print_issue_url(config: &mut GlobalConfig) {
    use bugreport::{bugreport, collector::*, format::Markdown};
    let other_bug_url: &str = "https://github.com/obi1kenobi/cargo-semver-checks/issues/new?labels=C-bug&template=3-bug-report.yml";

    let mut bug_report = bugreport!()
        .info(SoftwareVersion::default())
        .info(OperatingSystem::default())
        .info(CommandLine::default())
        .info(CommandOutput::new("cargo version", "cargo", &["-V"]))
        .info(CompileTimeInformation::default());

    let bold_cyan = Style::new()
        .bold()
        .fg_color(Some(Color::Ansi(AnsiColor::Cyan)));

    writeln!(
        config.stdout(),
        "{bold_cyan}\
        System information:{Reset}\n\
        -------------------"
    )
    .expect("Failed to print bug report system information to stdout");
    bug_report.print::<Markdown>();

    let bug_report = bug_report.format::<Markdown>();
    let bug_report_url = urlencoding::encode(&bug_report);

    let cargo_config = match Config::load() {
        Ok(c) => toml::to_string(&c).unwrap_or_else(|s| {
            writeln!(
                config.stderr(),
                "Error serializing cargo build configuration: {}",
                s
            )
            .expect("Failed to print error");
            String::default()
        }),
        Err(e) => {
            writeln!(
                config.stderr(),
                "Error loading cargo build configuration: {}",
                e
            )
            .expect("Failed to print error");
            String::default()
        }
    };

    writeln!(
        config.stdout(),
        "{bold_cyan}\
        Cargo build configuration:{Reset}\n\
        --------------------------\n\
        {cargo_config}"
    )
    .expect("Failed to print bug report Cargo configuration to stdout");

    let cargo_config_url: String = urlencoding::encode(&cargo_config).into_owned();

    let bold = Style::new().bold();
    writeln!(
        config.stdout(),
        "{bold}Please file an issue on GitHub reporting your bug.\n\
        Consider adding the diagnostic information above, either manually or automatically through the link below:{Reset}\n\n\
        {other_bug_url}&sys-info={bug_report_url}&build-config={cargo_config_url}",
    )
    .expect("Failed to print bug report generated github issue link");
}

#[derive(Debug, Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
#[command(version, propagate_version = true)]
enum Cargo {
    SemverChecks(SemverChecks),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct SemverChecks {
    #[arg(long, global = true, exclusive = true)]
    bugreport: bool,

    #[arg(long, global = true, exclusive = true)]
    explain: Option<String>,

    #[arg(long, global = true, exclusive = true)]
    list: bool,

    #[clap(flatten)]
    check_release: CheckRelease,

    #[command(subcommand)]
    command: Option<SemverChecksCommands>,

    // we need to use clap::ColorChoice instead of anstream::ColorChoice
    // because ValueEnum is implemented for it.
    /// Choose whether to output colors
    #[arg(long = "color", global = true, value_name = "WHEN", value_enum)]
    color_choice: Option<clap::ColorChoice>,

    // docstring for help is on the `clap_verbosity_flag::Verbosity` struct itself
    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity<clap_verbosity_flag::InfoLevel>,
}

/// Check your crate for semver violations.
#[derive(Debug, Subcommand)]
enum SemverChecksCommands {
    #[command(alias = "diff-files")]
    CheckRelease(CheckRelease),
}

#[derive(Debug, Args, Clone)]
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
        requires = "baseline_rustdoc",
        conflicts_with_all = [
            "default_features",
            "only_explicit_features",
            "features",
            "baseline_features",
            "current_features",
            "all_features",
        ]
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
        group = "baseline",
        conflicts_with_all = [
            "default_features",
            "only_explicit_features",
            "features",
            "baseline_features",
            "current_features",
            "all_features",
        ]
    )]
    baseline_rustdoc: Option<PathBuf>,

    /// Sets the release type instead of deriving it from the version number.
    #[arg(
        value_enum,
        long,
        value_name = "TYPE",
        help_heading = "Overrides",
        group = "overrides"
    )]
    release_type: Option<ReleaseType>,

    /// Use only the crate-defined default features, as well as any features
    /// added explicitly via other flags.
    ///
    /// Using this flag disables the heuristic that enables all features
    /// except `unstable`, `nightly`, `bench`, `no_std`, and ones starting with prefixes
    /// `_`, `unstable_`, `unstable-`.
    #[arg(
        long,
        help_heading = "Features",
        conflicts_with = "only_explicit_features"
    )]
    default_features: bool,

    /// Use no features except ones explicitly added by other flags.
    ///
    /// Using this flag disables the heuristic that enables all features
    /// except `unstable`, `nightly`, `bench`, `no_std`, and ones starting with prefixes
    /// `_`, `unstable_`, `unstable-`.
    #[arg(long, help_heading = "Features")]
    only_explicit_features: bool,

    /// Add a feature to the set of features being checked.
    /// The feature will be used in both the baseline and the current version
    /// of the crate.
    #[arg(long, value_name = "NAME", help_heading = "Features")]
    features: Vec<String>,

    /// Add a feature to the set of features being checked.
    /// The feature will be used in the baseline version of the crate only.
    #[arg(long, value_name = "NAME", help_heading = "Features")]
    baseline_features: Vec<String>,

    /// Add a feature to the set of features being checked.
    /// The feature will be used in the current version of the crate only.
    #[arg(long, value_name = "NAME", help_heading = "Features")]
    current_features: Vec<String>,

    /// Use all the features, including features named
    /// `unstable`, `nightly`, `bench`, `no_std` or starting with prefixes
    /// `_`, `unstable_`, `unstable-` that are otherwise disabled by default.
    #[arg(
        long,
        help_heading = "Features",
        conflicts_with_all = [
            "default_features",
            "only_explicit_features",
            "features",
            "baseline_features",
            "current_features",
        ]
    )]
    all_features: bool,

    /// Which target to build the crate for, to check platform-specific APIs, e.g.
    /// `x86_64-unknown-linux-gnu`.
    #[arg(long = "target")]
    build_target: Option<String>,
}

impl From<CheckRelease> for cargo_semver_checks::Check {
    fn from(value: CheckRelease) -> Self {
        let (current, current_project_root) = if let Some(current_rustdoc) = value.current_rustdoc {
            (Rustdoc::from_path(current_rustdoc), None)
        } else if let Some(manifest) = value.manifest.manifest_path {
            let project_root = if manifest.is_dir() {
                manifest
            } else {
                manifest
                    .parent()
                    .expect("manifest path doesn't have a parent")
                    .to_path_buf()
            };
            (Rustdoc::from_root(&project_root), Some(project_root))
        } else {
            let project_root = std::env::current_dir().expect("can't determine current directory");
            (Rustdoc::from_root(&project_root), Some(project_root))
        };
        let mut check = Self::new(current);
        if value.workspace.all || value.workspace.workspace {
            // Specified explicit `--workspace` or `--all`.
            let mut selection = PackageSelection::new(ScopeSelection::Workspace);
            selection.set_excluded_packages(value.workspace.exclude);
            check.set_package_selection(selection);
        } else if !value.workspace.package.is_empty() {
            // Specified explicit `--package`.
            check.set_packages(value.workspace.package);
        } else if !value.workspace.exclude.is_empty() {
            // Specified `--exclude` without `--workspace/--all`.
            // Leave the scope selection to the default ("workspace if the manifest is a workspace")
            // while excluding any specified packages.
            let mut selection = PackageSelection::new(ScopeSelection::DefaultMembers);
            selection.set_excluded_packages(value.workspace.exclude);
            check.set_package_selection(selection);
        }
        let custom_baseline = {
            if let Some(baseline_version) = value.baseline_version {
                Some(Rustdoc::from_registry(baseline_version))
            } else if let Some(baseline_rev) = value.baseline_rev {
                let root = if let Some(baseline_root) = value.baseline_root {
                    baseline_root
                } else if let Some(current_root) = current_project_root {
                    current_root
                } else {
                    std::env::current_dir().expect("can't determine current directory")
                };
                Some(Rustdoc::from_git_revision(root, baseline_rev))
            } else if let Some(baseline_rustdoc) = value.baseline_rustdoc {
                Some(Rustdoc::from_path(baseline_rustdoc))
            } else {
                // Either there's a manually-set baseline root path, or fall through
                // to the default behavior.
                value.baseline_root.map(Rustdoc::from_root)
            }
        };
        if let Some(baseline) = custom_baseline {
            check.set_baseline(baseline);
        }

        if let Some(release_type) = value.release_type {
            check.set_release_type(release_type);
        }

        if value.all_features {
            check.with_all_features();
        } else if value.default_features {
            check.with_default_features();
        } else if value.only_explicit_features {
            check.with_only_explicit_features();
        } else {
            check.with_heuristically_included_features();
        }
        let mut mutual_features = value.features;
        let mut current_features = value.current_features;
        let mut baseline_features = value.baseline_features;
        current_features.append(&mut mutual_features.clone());
        baseline_features.append(&mut mutual_features);

        // Treat --features="" as a no-op like cargo does
        let trim_features = |features: &mut Vec<String>| {
            features.retain(|feature| !(feature.is_empty() || feature == "\"\""));
        };
        trim_features(&mut current_features);
        trim_features(&mut baseline_features);

        check.set_extra_features(current_features, baseline_features);

        if let Some(build_target) = value.build_target {
            check.set_build_target(build_target);
        }

        check
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cargo::command().debug_assert()
}

#[test]
fn features_empty_string_is_no_op() {
    use cargo_semver_checks::Check;

    let Cargo::SemverChecks(SemverChecks {
        check_release: no_features,
        ..
    }) = Cargo::parse_from(["cargo", "semver-checks"]);

    let empty_features = CheckRelease {
        features: vec![String::new()],
        current_features: vec![String::new(), "\"\"".to_string()],
        baseline_features: vec!["\"\"".to_string()],
        ..no_features.clone()
    };

    assert_eq!(Check::from(no_features), Check::from(empty_features));
}
