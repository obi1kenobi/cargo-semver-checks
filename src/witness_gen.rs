use std::{collections::BTreeMap, sync::Arc};

use itertools::Itertools;
use trustfall::FieldValue;
use trustfall_rustdoc::VersionedRustdocAdapter;

use crate::query::WitnessQuery;

/// Runs the witness query of a given [`WitnessQuery`] for each lint query match, and merges the witness query
/// results with the existing lint results. Each query must match exactly once, and will fail with an
/// [`anyhow::Error`] otherwise.
///
/// Overlapping output keys between the [`WitnessQuery`] and the [`SemverQuery`](crate::query::SemverQuery)
/// will result in the result from the [`WitnessQuery`] silently overriding the same key from the
/// [`SemverQuery`](crate::query::SemverQuery).
pub(crate) fn run_witness_queries(
    adapter: &VersionedRustdocAdapter,
    witness_query: &WitnessQuery,
    lint_results: &Vec<BTreeMap<Arc<str>, FieldValue>>,
) -> Vec<anyhow::Result<BTreeMap<Arc<str>, FieldValue>>> {
    lint_results
        .clone()
        .into_iter()
        .map(|mut single_result| {
            let arguments = witness_query.inherit_arguments_from(&single_result)?;

            let witness_results = adapter
                .run_query(&witness_query.query, arguments)
                .and_then(|mut query_results| {
                    let query_result = query_results.next();

                    match query_results.next() {
                        // If there is an extra query match, we don't know which is the "correct one"
                        Some(_) => Err(anyhow::anyhow!(
                            "witness query should match exactly one time, matched multiple times"
                        )),
                        // If there is no query match, something has gone very wrong
                        None => query_result.ok_or(anyhow::anyhow!(
                            "witness query should match exactly one time, matched zero times"
                        )),
                    }
                })?;

            single_result.extend(witness_results);
            Ok(single_result)
        })
        .collect_vec()
}
