#![forbid(unsafe_code)]

pub mod adapter;
mod check_release;
pub mod indexed_crate;
mod query;
mod templating;
mod util;

use std::env;
use std::path::PathBuf;

use clap::{crate_version, value_parser, Arg, ArgAction, Command};
use termcolor::{ColorChoice, StandardStream};

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
    let matches = cmd().get_matches();

    // Descend one level: from `cargo semver-checks` to just `semver-checks`.
    let semver_check = matches
        .subcommand_matches("semver-checks")
        .expect("semver-checks is missing");

    let config = GlobalConfig::new();

    match semver_check.subcommand() {
        Some(("diff-files", diff_files)) => {
            let current_rustdoc_path = diff_files
                .get_one::<PathBuf>("current_rustdoc_path")
                .expect("current_rustdoc_path is required but was not present")
                .as_path();
            let baseline_rustdoc_path = diff_files
                .get_one::<PathBuf>("baseline_rustdoc_path")
                .expect("baseline_rustdoc_path is required but was not present")
                .as_path();

            let current_crate = load_rustdoc_from_file(current_rustdoc_path)?;
            let baseline_crate = load_rustdoc_from_file(baseline_rustdoc_path)?;

            return run_check_release(config, current_crate, baseline_crate);
        }
        Some(("check-release", check_release)) => {
            let current_rustdoc_path = check_release
                .get_one::<PathBuf>("current_rustdoc_path")
                .expect("current_rustdoc_path is required but was not present")
                .as_path();
            let baseline_rustdoc_path = check_release
                .get_one::<PathBuf>("baseline_rustdoc_path")
                .expect("baseline_rustdoc_path is required but was not present")
                .as_path();

            let current_crate = load_rustdoc_from_file(current_rustdoc_path)?;
            let baseline_crate = load_rustdoc_from_file(baseline_rustdoc_path)?;

            return run_check_release(config, current_crate, baseline_crate);
        }
        Some(_) => {
            unreachable!("external subcommands were not enabled with clap")
        }
        None => {
            unreachable!("arg_required_else_help is set with clap")
        }
    }
}

fn cmd() -> Command<'static> {
    Command::new("cargo-semver-checks")
        .bin_name("cargo")
        .version(crate_version!())
        .propagate_version(true)
        .subcommand(
            Command::new("semver-checks")
                .about("Check your crate for semver violations.")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("diff-files")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("current_rustdoc_path")
                                .short('c')
                                .long("current")
                                .value_name("CURRENT_RUSTDOC_JSON")
                                .help("The current rustdoc json output to test for semver violations. Required.")
                                .action(ArgAction::Set)
                                .value_parser(value_parser!(PathBuf))
                                .required(true)
                        )
                        .arg(
                            Arg::new("baseline_rustdoc_path")
                                .short('b')
                                .long("baseline")
                                .value_name("BASELINE_RUSTDOC_JSON")
                                .help("The rustdoc json file to use as a semver baseline. Required.")
                                .action(ArgAction::Set)
                                .value_parser(value_parser!(PathBuf))
                                .required(true)
                        )
                )
                .subcommand(
                    Command::new("check-release")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("current_rustdoc_path")
                                .short('c')
                                .long("current")
                                .value_name("CURRENT_RUSTDOC_JSON")
                                .help("The current rustdoc json output to test for semver violations. Required.")
                                .action(ArgAction::Set)
                                .value_parser(value_parser!(PathBuf))
                                .required(true)
                        )
                        .arg(
                            Arg::new("baseline_rustdoc_path")
                                .short('b')
                                .long("baseline")
                                .value_name("BASELINE_RUSTDOC_JSON")
                                .help("The rustdoc json file to use as a semver baseline. Required.")
                                .action(ArgAction::Set)
                                .value_parser(value_parser!(PathBuf))
                                .required(true)
                        )
                )
        )
}

#[test]
fn verify_cmd() {
    cmd().debug_assert();
}
