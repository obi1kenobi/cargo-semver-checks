#![forbid(unsafe_code)]

pub mod adapter;
mod query;

use adapter::RustdocAdapter;
use anyhow::Context;
use clap::{crate_version, AppSettings, Arg, Command};
use query::{SemverQuery, ActualSemverUpdate};
use rustdoc_types::Crate;
use std::{cell::RefCell, collections::BTreeMap, env, fs::File, io::Read, rc::Rc, sync::Arc};
use trustfall_core::{frontend::parse, interpreter::execution::interpret_ir, ir::TransparentValue};

use crate::query::RequiredSemverUpdate;

fn main() -> anyhow::Result<()> {
    let matches = Command::new("cargo-semver-checks")
        .bin_name("cargo")
        .version(crate_version!())
        .subcommand(
            Command::new("semver-checks")
                .version(crate_version!())
                .about("Check your crate for semver violations.")
                .subcommand(
                    Command::new("diff-files")
                        .version(crate_version!())
                        .setting(AppSettings::ArgRequiredElseHelp)
                        .arg(
                            Arg::with_name("current_rustdoc_path")
                                .long("current")
                                .value_name("CURRENT")
                                .help("The current rustdoc json output to test for semver violations.")
                                .takes_value(true)
                                .required(true)
                        )
                        .arg(
                            Arg::with_name("baseline_rustdoc_path")
                                .long("baseline")
                                .value_name("BASELINE")
                                .help("The rustdoc json file to use as a semver baseline.")
                                .takes_value(true)
                                .required(true)
                        )
                )
        ).get_matches();

    // Descend one level: from `cargo semver-checks` to just `semver-checks`.
    let semver_check = matches
        .subcommand_matches("semver-checks")
        .expect("semver-checks is missing");

    if let Some(diff_files) = semver_check.subcommand_matches("diff-files") {
        let current_rustdoc_path: &str = diff_files
            .get_one::<String>("current_rustdoc_path")
            .expect("current_rustdoc_path is required but was not present")
            .as_str();
        let baseline_rustdoc_path: &str = diff_files
            .get_one::<String>("baseline_rustdoc_path")
            .expect("baseline_rustdoc_path is required but was not present")
            .as_str();

        let current_crate = load_rustdoc_from_file(current_rustdoc_path)?;
        let baseline_crate = load_rustdoc_from_file(baseline_rustdoc_path)?;

        return handle_diff_files(current_crate, baseline_crate);
    }

    unreachable!("no commands matched")
}

fn get_semver_version_change(current_version: Option<&str>, baseline_version: Option<&str>) -> Option<ActualSemverUpdate> {
    if let (Some(baseline), Some(current)) = (baseline_version, current_version) {
        let baseline_version = semver::Version::parse(baseline).expect("baseline not a valid version");
        let current_version = semver::Version::parse(current).expect("current not a valid version");

        // From the cargo reference:
        // > Initial development releases starting with "0.y.z" can treat changes
        // > in "y" as a major release, and "z" as a minor release.
        // > "0.0.z" releases are always major changes. This is because Cargo uses
        // > the convention that only changes in the left-most non-zero component
        // > are considered incompatible.
        // https://doc.rust-lang.org/cargo/reference/semver.html
        let update_kind = if baseline_version.major != current_version.major {
            ActualSemverUpdate::Major
        } else if baseline_version.minor != current_version.minor {
            if current_version.major == 0 {
                ActualSemverUpdate::Major
            } else {
                ActualSemverUpdate::Minor
            }
        } else if baseline_version.patch != current_version.patch {
            if current_version.major == 0 {
                if current_version.minor == 0 {
                    ActualSemverUpdate::Major
                } else {
                    ActualSemverUpdate::Minor
                }
            } else {
                ActualSemverUpdate::Patch
            }
        } else {
            ActualSemverUpdate::NotChanged
        };

        Some(update_kind)
    } else {
        None
    }
}

fn handle_diff_files(current_crate: Crate, baseline_crate: Crate) -> anyhow::Result<()> {
    let current_version = current_crate.crate_version.as_deref();
    let baseline_version = baseline_crate.crate_version.as_deref();

    let version_change = get_semver_version_change(current_version, baseline_version).unwrap_or_else(|| {
        println!("> Could not determine whether crate version changed. Assuming no change.");
        ActualSemverUpdate::NotChanged
    });
    let change = match version_change {
        ActualSemverUpdate::Major => "major",
        ActualSemverUpdate::Minor => "minor",
        ActualSemverUpdate::Patch => "patch",
        ActualSemverUpdate::NotChanged => "no",
    };
    println!("> Crate version {} -> {} ({} change)", baseline_version.unwrap_or("unknown"), current_version.unwrap_or("unknown"), change);

    let queries = SemverQuery::all_queries();

    let schema = RustdocAdapter::schema();
    let adapter = Rc::new(RefCell::new(RustdocAdapter::new(
        &current_crate,
        Some(&baseline_crate),
    )));
    let mut found_errors = false;

    for semver_query in queries.values() {
        if version_change.supports_requirement(semver_query.required_update) {
            println!("> Skipping allowed change: {}", &semver_query.human_readable_name);
            continue;
        }

        let parsed_query = parse(&schema, &semver_query.query)
            .expect("not a valid query, should have been caught in tests");
        let args = Arc::new(
            semver_query
                .arguments
                .iter()
                .map(|(k, v)| (Arc::from(k.clone()), v.clone().into()))
                .collect(),
        );
        let mut results_iter = interpret_ir(adapter.clone(), parsed_query, args)
            .with_context(|| "Query execution error.")?
            .peekable();

        let start_instant = std::time::Instant::now();
        print!("> Checking: {} ... ", &semver_query.human_readable_name);
        if results_iter.peek().is_none() {
            let end_instant = std::time::Instant::now();
            println!("OK ({:.3}s)", (end_instant - start_instant).as_secs_f32());
        } else {
            found_errors = true;
            let version_bump_needed = match semver_query.required_update {
                RequiredSemverUpdate::Major => "major",
                RequiredSemverUpdate::Minor => "minor",
            };
            println!("NOT OK: needs {} version\n", version_bump_needed);
            for semver_violation_result in results_iter.take(5) {
                let pretty_result: BTreeMap<Arc<str>, TransparentValue> = semver_violation_result
                    .into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect();
                println!("{}\n", serde_json::to_string_pretty(&pretty_result)?);
            }
        }
    }

    if found_errors {
        println!("> Done, found errors.");
        std::process::exit(1);
    }

    println!("> Done, no errors.");
    Ok(())
}

fn load_rustdoc_from_file(path: &str) -> anyhow::Result<Crate> {
    // Parsing JSON after fully reading a file into memory is much faster than
    // parsing directly from a file, even if buffered:
    // https://github.com/serde-rs/json/issues/160
    let mut s = String::new();
    File::open(path)
        .with_context(|| format!("Failed to open rustdoc JSON output file {:?}", path))?
        .read_to_string(&mut s)
        .with_context(|| format!("Failed to read rustdoc JSON output file {:?}", path))?;

    serde_json::from_str(&s)
        .with_context(|| format!("Failed to parse rustdoc JSON output file {:?}", path))
}
