use std::{collections::BTreeMap, path::Path, sync::Arc, time::Duration};

use anyhow::{Context, Result};
use handlebars::Handlebars;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use trustfall::TransparentValue;
use trustfall_rustdoc::VersionedRustdocAdapter;

use crate::{
    query::{QueryResults, Witness, WitnessQuery},
    GlobalConfig, SemverQuery,
};

/// Runs the witness query of a given [`WitnessQuery`] a given lint query match, and merges the witness query
/// results with the existing lint results. Each query must match exactly once, and will fail with an
/// [`anyhow::Error`] otherwise.
///
/// Overlapping output keys between the [`WitnessQuery`] and the [`SemverQuery`](crate::query::SemverQuery)
/// will result in the result from the [`WitnessQuery`] silently overriding the same key from the
/// [`SemverQuery`](crate::query::SemverQuery).
fn run_witness_query(
    adapter: &VersionedRustdocAdapter,
    witness_query: &WitnessQuery,
    mut lint_result: QueryResults,
) -> Result<QueryResults> {
    let arguments = witness_query
        .inherit_arguments_from(&lint_result)
        .context("Error inheriting arguments in witness query")?;

    let witness_results = adapter
        .run_query(&witness_query.query, arguments.clone())
        .and_then(|mut query_results| {
            if let Some(query_result) = query_results.next() {
                match query_results.next() {
                    // If there is an extra query match, we don't know which is the "correct one"
                    Some(extra_match) => Err(anyhow::anyhow!(
                        "witness query should match exactly one time, query matched producing both {:?} and {:?}", query_result, extra_match
                    )),
                    None => Ok(query_result),
                }
            } else {
                // If there is no query match, something has gone very wrong
                Err(anyhow::anyhow!(
                    "witness query should match exactly one time, matched zero times"
                ))
            }
        })
        .with_context(|| {
            format!(
                "Error running witness query with input arguments {:?}",
                arguments
            )
        })?;

    lint_result.extend(witness_results);
    Ok(lint_result)
}

fn generate_witness_text(
    handlebars: &Handlebars,
    witness_template: &str,
    witness_results: QueryResults,
) -> Result<String> {
    let pretty_witness_data: BTreeMap<Arc<str>, TransparentValue> = witness_results
        .into_iter()
        .map(|(k, v)| (k, v.into()))
        .collect();

    handlebars
        .render_template(witness_template, &pretty_witness_data)
        .context("Error instantiating witness template.")
}

fn map_to_witness_text<'query>(
    handlebars: &Handlebars,
    semver_query: &'query SemverQuery,
    lint_results: &[QueryResults],
    adapter: &VersionedRustdocAdapter,
) -> Option<(&'query SemverQuery, Vec<Result<String>>)> {
    match semver_query.witness {
        // Don't bother running the witness query unless both a witness query and template exist
        Some(Witness {
            witness_template: Some(ref witness_template),
            witness_query: Some(ref witness_query),
            ..
        }) => {
            let witness_results = lint_results
                .iter()
                .cloned()
                .map(|lint_result| {
                    let witness_results = run_witness_query(adapter, witness_query, lint_result)
                        .with_context(|| {
                            format!(
                                "Error running witness query for {}: {}",
                                semver_query.id, semver_query.human_readable_name
                            )
                        })?;
                    generate_witness_text(handlebars, witness_template, witness_results)
                        .with_context(|| {
                            format!(
                                "Error generating witness text for witness {}: {}",
                                semver_query.id, semver_query.human_readable_name
                            )
                        })
                })
                .collect_vec();
            Some((semver_query, witness_results))
        }

        // If no witness query exists, we still want to forward the existing output
        Some(Witness {
            witness_template: Some(ref witness_template),
            witness_query: None,
            ..
        }) => Some((
            semver_query,
            lint_results
                .iter()
                .cloned()
                .map(|lint_result| {
                    generate_witness_text(handlebars, witness_template, lint_result).with_context(
                        || {
                            format!(
                                "Error generating witness text for queryless witness {}: {}",
                                semver_query.id, semver_query.human_readable_name
                            )
                        },
                    )
                })
                .collect_vec(),
        )),
        _ => None,
    }
}

pub(crate) fn run_witness_checks(
    config: &GlobalConfig,
    _witness_dir: &Path,
    adapter: &VersionedRustdocAdapter,
    lint_results: &[(&SemverQuery, Duration, Vec<QueryResults>)],
) {
    // Have to pull out handlebars, since &GlobalConfig cannot be shared across threads
    let handlebars = config.handlebars();

    let _ = lint_results
        .par_iter()
        .filter_map(|(semver_query, _, query_results)| {
            map_to_witness_text(handlebars, semver_query, query_results, adapter)
        });
}
