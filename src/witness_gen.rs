use std::{
    collections::{BTreeMap, btree_map},
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context, Result};
use handlebars::Handlebars;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use trustfall::{FieldValue, TransparentValue};
use trustfall_rustdoc::VersionedRustdocAdapter;

use crate::check_release::LintResult;
use crate::data_generation::CrateDataRequest;
use crate::query::{Witness, WitnessQuery};
use crate::{GlobalConfig, SemverQuery};

/// Higher level data used specifically in the generation of a witness program. Values of [`None`] will
/// prevent witness generation, since the data is required for witness generation.
pub(crate) struct WitnessGenerationData<'a> {
    baseline: Option<&'a CrateDataRequest<'a>>,
    current: Option<&'a CrateDataRequest<'a>>,
    target_dir: Option<&'a Path>,
}

impl<'a> WitnessGenerationData<'a> {
    pub(crate) fn new(
        baseline: Option<&'a CrateDataRequest<'a>>,
        current: Option<&'a CrateDataRequest<'a>>,
        target_dir: Option<&'a Path>,
    ) -> Self {
        Self {
            baseline,
            current,
            target_dir,
        }
    }
}

/// Runs the witness query of a given [`WitnessQuery`] a given lint query match, and merges the witness query
/// results with the existing lint results. Each query must match exactly once, and will fail with an
/// [`anyhow::Error`] otherwise.
///
/// Overlapping output keys between the [`WitnessQuery`] and the [`SemverQuery`]
/// will result in an error.
fn run_witness_query(
    adapter: &VersionedRustdocAdapter,
    witness_query: &WitnessQuery,
    mut lint_result: BTreeMap<Arc<str>, FieldValue>,
) -> Result<BTreeMap<Arc<str>, FieldValue>> {
    let arguments = witness_query
        .inherit_arguments_from(&lint_result)
        .context("Error inheriting arguments in witness query")?;

    let witness_results = adapter
        .run_query(&witness_query.query, arguments)
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
                "error running witness query with input arguments {:?}",
                witness_query.inherit_arguments_from(&lint_result).expect("failed to reconstruct witness query arguments while creating error")
            )
        })?;

    for (key, value) in witness_results {
        match lint_result.entry(key) {
            btree_map::Entry::Vacant(entry) => {
                entry.insert(value);
            }
            btree_map::Entry::Occupied(entry) => anyhow::bail!(
                "witness query tried to output to existing key `{}`, overriding `{:?}` with `{:?}`",
                entry.key(),
                entry.get(),
                value,
            ),
        }
    }

    Ok(lint_result)
}

fn generate_witness_text(
    handlebars: &Handlebars,
    witness_template: &str,
    witness_results: BTreeMap<Arc<str>, FieldValue>,
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
    lint_results: &[BTreeMap<Arc<str>, FieldValue>],
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
                            format!("error running witness query for {}", semver_query.id)
                        })?;
                    generate_witness_text(handlebars, witness_template, witness_results)
                        .with_context(|| {
                            format!(
                                "error generating witness text for witness {}",
                                semver_query.id
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
                                "error generating witness text for queryless witness {}",
                                semver_query.id
                            )
                        },
                    )
                })
                .collect_vec(),
        )),
        _ => None,
    }
}

/// Generates a single witness crate
fn generate_witness_crate(
    witness_set_dir: &Path,
    witness_name: &str,
    index: usize,
    _witness_text: String,
) -> Result<PathBuf> {
    let crate_path = witness_set_dir.join(format!("{witness_name}-{index}"));
    fs::create_dir_all(&crate_path)
        .with_context(|| format!("error creating witness at `{crate_path:?}`",))?;

    // TODO: Finish crate generation, currently just generates an empty dir

    Ok(crate_path)
}

/// Utility for printing a warning message
fn print_warning(config: &mut GlobalConfig, msg: impl std::fmt::Display) {
    // Ignore terminal printing errors
    let _ = config.log_info(|config| {
        config.shell_warn(msg)?;
        Ok(())
    });
}

pub(crate) fn run_witness_checks(
    config: &mut GlobalConfig,
    witness_data: WitnessGenerationData,
    crate_name: &str,
    adapter: &VersionedRustdocAdapter,
    lint_results: &[LintResult],
) {
    let (_baseline_data, _current_data, target_dir) = match witness_data {
        WitnessGenerationData {
            baseline: Some(baseline),
            current: Some(current),
            target_dir: Some(target_dir),
        } => (baseline, current, target_dir),
        _ => {
            print_warning(
                config,
                format!(
                    "encountered non-fatal error while creating witness program \
                    {crate_name}: cannot process witness for this source type (root cause: witness data is not complete)",
                ),
            );
            return;
        }
    };

    let witness_set_dir = target_dir.join(format!("{}-{crate_name}", config.run_id()));

    // Have to pull out handlebars, since &GlobalConfig cannot be shared across threads
    let handlebars = config.handlebars();

    let _ = lint_results
        .par_iter()
        .filter_map(|res| {
            map_to_witness_text(handlebars, &res.semver_query, &res.query_results, adapter)
        })
        .flat_map(|(semver_query, witness_texts)| {
            witness_texts
                .into_iter()
                .enumerate()
                .map(|(index, witness_text)| {
                    generate_witness_crate(&witness_set_dir, &semver_query.id, index, witness_text?)
                })
                // This collect is necessary to convert the above synchronous Iter into a ParIter
                .collect_vec()
        });
}
