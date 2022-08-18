use std::{
    cell::RefCell, collections::BTreeMap, env, io::Write, iter::Peekable, rc::Rc, sync::Arc,
    time::Duration,
};

use anyhow::Context;
use clap::crate_version;
use rustdoc_types::Crate;
use termcolor::Color;
use termcolor_output::{colored, colored_ln};
use trustfall_core::{
    frontend::parse,
    interpreter::execution::interpret_ir,
    ir::{FieldValue, TransparentValue},
    schema::Schema,
};

use crate::{
    adapter::RustdocAdapter,
    indexed_crate::IndexedCrate,
    query::{ActualSemverUpdate, RequiredSemverUpdate, SemverQuery},
    GlobalConfig,
};

type QueryResultItem = BTreeMap<Arc<str>, FieldValue>;

struct QueryWithResults<'a> {
    name: &'a str,
    results: Peekable<Box<dyn Iterator<Item = QueryResultItem> + 'a>>,
}

impl<'a> QueryWithResults<'a> {
    fn new(
        name: &'a str,
        results: Peekable<Box<dyn Iterator<Item = QueryResultItem> + 'a>>,
    ) -> Self {
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

fn make_result_iter<'a>(
    schema: &Schema,
    adapter: Rc<RefCell<RustdocAdapter<'a>>>,
    semver_query: &SemverQuery,
) -> anyhow::Result<Peekable<Box<dyn Iterator<Item = QueryResultItem> + 'a>>> {
    let parsed_query = parse(schema, &semver_query.query)
        .expect("not a valid query, should have been caught in tests");
    let args = Arc::new(
        semver_query
            .arguments
            .iter()
            .map(|(k, v)| (Arc::from(k.clone()), v.clone().into()))
            .collect(),
    );
    let results_iter = interpret_ir(adapter.clone(), parsed_query, args)
        .with_context(|| "Query execution error.")?
        .peekable();

    Ok(results_iter)
}

pub(super) fn run_check_release(
    config: &mut GlobalConfig,
    crate_name: &str,
    current_crate: Crate,
    baseline_crate: Crate,
) -> anyhow::Result<bool> {
    let current_version = current_crate.crate_version.as_deref();
    let baseline_version = baseline_crate.crate_version.as_deref();

    let version_change = get_semver_version_change(current_version, baseline_version)
        .unwrap_or_else(|| {
            config
                .shell_warn(
                    "Could not determine whether crate version changed. Assuming no change.",
                )
                .expect("print failed");
            ActualSemverUpdate::NotChanged
        });
    let change = match version_change {
        ActualSemverUpdate::Major => "major",
        ActualSemverUpdate::Minor => "minor",
        ActualSemverUpdate::Patch => "patch",
        ActualSemverUpdate::NotChanged => "no",
    };

    let queries = SemverQuery::all_queries();

    let schema = RustdocAdapter::schema();
    let current = IndexedCrate::new(&current_crate);
    let previous = IndexedCrate::new(&baseline_crate);
    let adapter = Rc::new(RefCell::new(RustdocAdapter::new(&current, Some(&previous))));
    let mut queries_with_errors: Vec<QueryWithResults> = vec![];

    let queries_to_run: Vec<_> = queries
        .iter()
        .filter(|(_, query)| !version_change.supports_requirement(query.required_update))
        .collect();
    let skipped_queries = queries.len().saturating_sub(queries_to_run.len());

    config.shell_status(
        "Checking",
        format_args!(
            "{crate_name} v{} -> v{} ({} change)",
            baseline_version.unwrap_or("unknown"),
            current_version.unwrap_or("unknown"),
            change
        ),
    )?;
    config
        .verbose(|config| {
            config.shell_status(
                "Starting",
                format_args!(
                    "{} checks, {} skipped",
                    queries_to_run.len(),
                    skipped_queries
                ),
            )
        })
        .expect("print failed");
    let mut total_duration = Duration::default();

    for (query_id, semver_query) in queries_to_run.iter().copied() {
        let category = match semver_query.required_update {
            RequiredSemverUpdate::Major => "major",
            RequiredSemverUpdate::Minor => "minor",
        };
        config
            .verbose(|config| {
                if config.is_stderr_tty() {
                    colored!(
                        config.stderr(),
                        "{}{}{:>12}{} [{:9}] {:^18} {}",
                        fg!(Some(Color::Cyan)),
                        bold!(true),
                        "Running",
                        reset!(),
                        "",
                        category,
                        query_id,
                    )?;
                    config.stderr().flush()?;
                }
                Ok(())
            })
            .expect("print failed");

        let start_instant = std::time::Instant::now();
        let mut results_iter = make_result_iter(&schema, adapter.clone(), semver_query)?;
        let peeked = results_iter.peek();
        let end_instant = std::time::Instant::now();
        let time_to_decide = end_instant - start_instant;
        total_duration += time_to_decide;

        if peeked.is_none() {
            config
                .verbose(|config| {
                    if config.is_stderr_tty() {
                        write!(config.stderr(), "\r")?;
                    }
                    colored_ln(config.stderr(), |w| {
                        colored!(
                            w,
                            "{}{}{:>12}{} [{:>8.3}s] {:^18} {}",
                            fg!(Some(Color::Green)),
                            bold!(true),
                            "PASS",
                            reset!(),
                            time_to_decide.as_secs_f32(),
                            category,
                            query_id,
                        )
                    })?;
                    Ok(())
                })
                .expect("print failed");
        } else {
            queries_with_errors.push(QueryWithResults::new(query_id.as_str(), results_iter));

            config
                .verbose(|config| {
                    if config.is_stderr_tty() {
                        write!(config.stderr(), "\r")?;
                    }
                    colored_ln(config.stderr(), |w| {
                        colored!(
                            w,
                            "{}{}{:>12}{} [{:>8.3}s] {:^18} {}",
                            fg!(Some(Color::Red)),
                            bold!(true),
                            "FAIL",
                            reset!(),
                            time_to_decide.as_secs_f32(),
                            category,
                            query_id,
                        )
                    })?;
                    Ok(())
                })
                .expect("print failed");
        }
    }

    if !queries_with_errors.is_empty() {
        config
            .shell_print(
                "Completed",
                format_args!(
                    "[{:>8.3}s] {} checks; {} passed, {} failed, {} skipped",
                    total_duration.as_secs_f32(),
                    queries_to_run.len(),
                    queries_to_run.len() - queries_with_errors.len(),
                    queries_with_errors.len(),
                    skipped_queries,
                ),
                Color::Red,
                true,
            )
            .expect("print failed");

        let mut required_versions = vec![];

        for query_with_results in queries_with_errors {
            let semver_query = &queries[query_with_results.name];
            required_versions.push(semver_query.required_update);
            colored_ln(config.stdout(), |w| {
                colored!(
                    w,
                    "\n--- failure {}: {} ---\n",
                    &semver_query.id,
                    &semver_query.human_readable_name,
                )
            })
            .expect("print failed");

            if let Some(ref_link) = semver_query.reference_link.as_deref() {
                colored_ln(config.stdout(), |w| {
                    colored!(
                        w,
                        "{}Description:{}\n{}\n{:>12} {}\n{:>12} {}\n",
                        bold!(true),
                        reset!(),
                        &semver_query.error_message,
                        "ref:",
                        ref_link,
                        "impl:",
                        format!(
                            "https://github.com/obi1kenobi/cargo-semver-check/tree/v{}/src/queries/{}.ron",
                            crate_version!(),
                            semver_query.id,
                        )
                    )
                })
                .expect("print failed");
            } else {
                colored_ln(config.stdout(), |w| {
                    colored!(
                        w,
                        "{}Description:{}\n{}\n{:>12} {}\n",
                        bold!(true),
                        reset!(),
                        &semver_query.error_message,
                        "impl:",
                        format!(
                            "https://github.com/obi1kenobi/cargo-semver-check/tree/v{}/src/queries/{}.ron",
                            crate_version!(),
                            semver_query.id,
                        )
                    )
                })
                .expect("print failed");
            }

            colored_ln(config.stdout(), |w| {
                colored!(w, "{}Failed in:{}", bold!(true), reset!(),)
            })
            .expect("print failed");

            let start_instant = std::time::Instant::now();
            for semver_violation_result in query_with_results.results {
                let pretty_result: BTreeMap<Arc<str>, TransparentValue> = semver_violation_result
                    .into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect();

                if let Some(template) = semver_query.per_result_error_template.as_deref() {
                    let message = config
                        .handlebars()
                        .render_template(template, &pretty_result)
                        .with_context(|| "Error instantiating semver query template.")
                        .expect("could not materialize template");
                    colored_ln(config.stdout(), |w| colored!(w, "  {}", message,))
                        .expect("print failed");
                } else {
                    colored_ln(config.stdout(), |w| {
                        colored!(
                            w,
                            "{}\n",
                            serde_json::to_string_pretty(&pretty_result).expect("serde failed"),
                        )
                    })
                    .expect("print failed");
                }
            }
            let end_instant = std::time::Instant::now();
            total_duration += end_instant - start_instant;
        }

        let required_bump = if required_versions.contains(&RequiredSemverUpdate::Major) {
            "major"
        } else if required_versions.contains(&RequiredSemverUpdate::Minor) {
            "minor"
        } else {
            unreachable!("{:?}", required_versions)
        };

        config
            .shell_print(
                "Final",
                format_args!(
                    "[{:>8.3}s] semver requires new {} version: {} major and {} minor checks failed",
                    total_duration.as_secs_f32(),
                    required_bump,
                    required_versions.iter().filter(|x| *x == &RequiredSemverUpdate::Major).count(),
                    required_versions.iter().filter(|x| *x == &RequiredSemverUpdate::Minor).count(),
                ),
                Color::Red,
                true,
            )
            .expect("print failed");

        Ok(false)
    } else {
        config
            .shell_print(
                "Completed",
                format_args!(
                    "[{:>8.3}s] {} checks; {} passed, {} skipped",
                    total_duration.as_secs_f32(),
                    queries_to_run.len(),
                    queries_to_run.len(),
                    skipped_queries,
                ),
                Color::Green,
                true,
            )
            .expect("print failed");
        Ok(true)
    }
}
