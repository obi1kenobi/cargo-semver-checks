use std::{cell::RefCell, collections::BTreeMap, env, rc::Rc, sync::Arc, iter::Peekable};

use anyhow::Context;
use clap::crate_version;
use handlebars::Handlebars;
use rustdoc_types::Crate;
use termcolor::Color;
use termcolor_output::{colored, colored_ln};
use trustfall_core::{
    frontend::parse,
    interpreter::execution::interpret_ir,
    ir::{FieldValue, TransparentValue},
};

use crate::{
    adapter::RustdocAdapter,
    query::{ActualSemverUpdate, RequiredSemverUpdate, SemverQuery},
    GlobalConfig,
};

type QueryResultItem = BTreeMap<Arc<str>, FieldValue>;

struct QueryWithResults<'a> {
    name: &'a str,
    results: Peekable<Box<dyn Iterator<Item = QueryResultItem> + 'a>>,
}

impl<'a> QueryWithResults<'a> {
    fn new(name: &'a str, results: Peekable<Box<dyn Iterator<Item = QueryResultItem> + 'a>>) -> Self {
        Self { name, results }
    }
}

fn get_semver_version_change(
    current_version: Option<&str>,
    baseline_version: Option<&str>,
) -> Option<ActualSemverUpdate> {
    if let (Some(baseline), Some(current)) = (baseline_version, current_version) {
        let baseline_version =
            semver::Version::parse(baseline).expect("baseline not a valid version");
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

pub(super) fn run_check_release(
    mut config: GlobalConfig,
    current_crate: Crate,
    baseline_crate: Crate,
) -> anyhow::Result<()> {
    let current_version = current_crate.crate_version.as_deref();
    let baseline_version = baseline_crate.crate_version.as_deref();

    let version_change = get_semver_version_change(current_version, baseline_version)
        .unwrap_or_else(|| {
            colored_ln(&mut config.output_writer, |w| {
                colored!(
                    w,
                    "{}{}{:>12}{} Could not determine whether crate version changed. Assuming no change.",
                    fg!(Some(Color::Yellow)),
                    bold!(true),
                    "Warning",
                    reset!(),
                )
            }).expect("print failed");
            ActualSemverUpdate::NotChanged
        });
    let change = match version_change {
        ActualSemverUpdate::Major => "major",
        ActualSemverUpdate::Minor => "minor",
        ActualSemverUpdate::Patch => "patch",
        ActualSemverUpdate::NotChanged => "no",
    };

    colored_ln(&mut config.output_writer, |w| {
        colored!(
            w,
            "{}{}{:>12}{} Crate version {} -> {} ({} change)",
            fg!(Some(Color::Cyan)),
            bold!(true),
            "Info",
            reset!(),
            baseline_version.unwrap_or("unknown"),
            current_version.unwrap_or("unknown"),
            change,
        )
    })
    .expect("print failed");

    let queries = SemverQuery::all_queries();

    let schema = RustdocAdapter::schema();
    let adapter = Rc::new(RefCell::new(RustdocAdapter::new(
        &current_crate,
        Some(&baseline_crate),
    )));
    let mut queries_with_errors: Vec<QueryWithResults> = vec![];

    for (query_id, semver_query) in queries.iter() {
        if version_change.supports_requirement(semver_query.required_update) {
            colored_ln(&mut config.output_writer, |w| {
                colored!(
                    w,
                    "{}{}{:>12}{} Allowed change: {}",
                    fg!(Some(Color::Green)),
                    bold!(true),
                    "Skipping",
                    reset!(),
                    &semver_query.human_readable_name,
                )
            })
            .expect("print failed");
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

        colored!(
            config.output_writer,
            "{}{}{:>12}{} {} ... ",
            fg!(Some(Color::Green)),
            bold!(true),
            "Checking",
            reset!(),
            &semver_query.human_readable_name,
        )
        .expect("print failed");

        let start_instant = std::time::Instant::now();
        if results_iter.peek().is_none() {
            let end_instant = std::time::Instant::now();
            colored!(
                config.output_writer,
                "{}{}{}{} ({:.3}s)\n",
                fg!(Some(Color::Green)),
                bold!(true),
                "OK",
                reset!(),
                (end_instant - start_instant).as_secs_f32(),
            )
            .expect("print failed");
        } else {
            queries_with_errors.push(QueryWithResults::new(query_id.as_str(), results_iter));

            let version_bump_needed = match semver_query.required_update {
                RequiredSemverUpdate::Major => "major",
                RequiredSemverUpdate::Minor => "minor",
            };
            colored!(
                config.output_writer,
                "{}{}{}{}\n",
                fg!(Some(Color::Red)),
                bold!(true),
                "NOT OK:",
                reset!(),
            )
            .expect("print failed");

            colored_ln(&mut config.output_writer, |w| {
                colored!(
                    w,
                    "{}{}{:>12}{} {}",
                    fg!(Some(Color::Cyan)),
                    bold!(true),
                    "Description",
                    reset!(),
                    semver_query.error_message,
                )
            })
            .expect("print failed");

            if let Some(ref_link) = semver_query.reference_link.as_deref() {
                colored_ln(&mut config.output_writer, |w| {
                    colored!(
                        w,
                        "{}{}{:>12}{} {}",
                        fg!(Some(Color::Cyan)),
                        bold!(true),
                        "Reference",
                        reset!(),
                        ref_link,
                    )
                })
                .expect("print failed");
            }
            colored_ln(&mut config.output_writer, |w| {
                colored!(
                    w,
                    "{}{}{:>12}{} {}",
                    fg!(Some(Color::Cyan)),
                    bold!(true),
                    "Implemented",
                    reset!(),
                    format!(
                        "https://github.com/obi1kenobi/cargo-semver-check/tree/v{}/src/queries/{}.ron",
                        crate_version!(),
                        semver_query.id,
                    ),
                )
            })
            .expect("print failed");

            let reg = Handlebars::new();
            for semver_violation_result in results_iter.take(5) {
                let pretty_result: BTreeMap<Arc<str>, TransparentValue> = semver_violation_result
                    .into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect();

                if let Some(template) = semver_query.per_result_error_template.as_deref() {
                    colored_ln(&mut config.output_writer, |w| {
                        colored!(
                            w,
                            "{}{}{:>12}{} {}",
                            fg!(Some(Color::Blue)),
                            bold!(true),
                            "Err Instance",
                            reset!(),
                            reg.render_template(template, &pretty_result)
                                .with_context(|| "Error instantiating semver query template.")
                                .expect("could not materialize template"),
                        )
                    })
                    .expect("print failed");
                } else {
                    colored_ln(&mut config.output_writer, |w| {
                        colored!(
                            w,
                            "{}{}{:>12}{} raw violation data: {}",
                            fg!(Some(Color::Cyan)),
                            bold!(true),
                            "Instance",
                            reset!(),
                            serde_json::to_string_pretty(&pretty_result).expect("serde failed"),
                        )
                    })
                    .expect("print failed");
                }
            }

            colored_ln(&mut config.output_writer, |w| {
                colored!(
                    w,
                    "{}{}{:>12}{} {}: requires {} version",
                    fg!(Some(Color::Red)),
                    bold!(true),
                    "Failed",
                    reset!(),
                    &semver_query.human_readable_name,
                    version_bump_needed,
                )
            })
            .expect("print failed");
        }
    }

    if !queries_with_errors.is_empty() {
        colored_ln(&mut config.output_writer, |w| {
            colored!(
                w,
                "{}{}{:>12}{} Found errors.",
                fg!(Some(Color::Red)),
                bold!(true),
                "Done",
                reset!(),
            )
        })
        .expect("print failed");

        std::process::exit(1);
    }

    colored_ln(&mut config.output_writer, |w| {
        colored!(
            w,
            "{}{}{:>12}{} No errors.",
            fg!(Some(Color::Green)),
            bold!(true),
            "Done",
            reset!(),
        )
    })
    .expect("print failed");

    Ok(())
}
