use std::{
    collections::{BTreeMap, btree_map},
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context, Result};
use handlebars::Handlebars;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use trustfall::{FieldValue, TransparentValue};
use trustfall_rustdoc::VersionedRustdocAdapter;

use crate::data_generation::{CrateDataRequest, ProjectRequest, RegistryRequest};
use crate::query::{Witness, WitnessQuery};
use crate::{GlobalConfig, SemverQuery};
use crate::{check_release::LintResult, data_generation::RequestKind};

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

enum DependencyKind {
    Registry { version: String },
    Local { path: PathBuf },
}

struct WitnessDependency {
    name: String,
    kind: DependencyKind,
}

impl TryFrom<&CrateDataRequest<'_>> for WitnessDependency {
    type Error = anyhow::Error;

    fn try_from(value: &CrateDataRequest<'_>) -> std::result::Result<Self, Self::Error> {
        let this = match value.kind {
            RequestKind::Registry(RegistryRequest { index_entry, .. }) => Self {
                name: index_entry.name.to_string(),
                kind: DependencyKind::Registry {
                    version: index_entry.version.to_string(),
                },
            },
            RequestKind::LocalProject(ProjectRequest { manifest, .. }) => Self {
                name: manifest
                    .parsed
                    .package
                    .as_ref()
                    .map(|pkg| pkg.name.clone())
                    .context("error retrieving manifest package, none is present")?,
                kind: DependencyKind::Local {
                    path: fs::canonicalize(
                        manifest
                            .path
                            .parent()
                            .context("error retrieving manifest path parent")?,
                    )
                    .context("error converting local path to absolute path")?,
                },
            },
        };

        Ok(this)
    }
}

impl WitnessDependency {
    fn to_dependency_string(&self) -> String {
        match &self.kind {
            DependencyKind::Registry { version } => {
                format!(r#"{{ version = "{version}", package = "{}"}}"#, self.name)
            }
            DependencyKind::Local { path } => format!(
                r#"{{ path = "{}", package = "{}" }}"#,
                path.display(),
                self.name
            ),
        }
    }
}

struct DependencyData {
    target_dir: PathBuf,
    baseline: WitnessDependency,
    current: WitnessDependency,
}

impl DependencyData {
    fn parse_input(witness_data: WitnessGenerationData) -> Result<Self> {
        let baseline_data = witness_data
            .baseline
            .context("error parsing witness data, missing baseline crate data")?;
        let current_data = witness_data
            .current
            .context("error parsing witness data, missing current crate data")?;
        let target_dir = witness_data
            .target_dir
            .map(|path| path.to_path_buf())
            .context("error parsing witness data, missing target directory")?;

        let baseline = baseline_data
            .try_into()
            .context("error parsing baseline dependency")?;
        let current = current_data
            .try_into()
            .context("error parsing baseline dependency")?;

        Ok(Self {
            target_dir,
            baseline,
            current,
        })
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
        .context("error inheriting arguments in witness query")?;

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
    witness_text: &str,
    dependency_data: &DependencyData,
) -> Result<PathBuf> {
    let crate_path = witness_set_dir.join(format!("{witness_name}-{index}"));
    let src_path = crate_path.join("src");

    fs::create_dir_all(&src_path)
        .with_context(|| format!("failed to create all directories in the path {src_path:?}",))?;

    let mut manifest_file = fs::File::create(crate_path.join("Cargo.toml")).with_context(|| {
        format!(
            "failed to create witness crate cargo manifest {:?}",
            crate_path.join("Cargo.toml")
        )
    })?;

    let mut lib_file = fs::File::create(src_path.join("lib.rs")).with_context(|| {
        format!(
            "failed to create main witness file {:?}",
            src_path.join("lib.rs")
        )
    })?;

    write!(
        manifest_file,
        r#"[package]
name = "{witness_name}-{index}"
version = "1.0.0"
edition = "2024"

[lib]
path = "src/lib.rs"

[features]
baseline = []

[dependencies]
baseline = {}
current = {}
"#,
        dependency_data.baseline.to_dependency_string(),
        dependency_data.current.to_dependency_string()
    )
    .context("error writing to manifest file")?;

    write!(
        lib_file,
        r#"#[cfg(feature = "baseline")]
use baseline as {0};
#[cfg(not(feature = "baseline"))]
use current as {0};

{1}
"#,
        dependency_data.baseline.name, witness_text
    )
    .context("error writing to main witness file")?;

    Ok(crate_path)
}

/// Runs a single witness check, returning the result of the witness process
///
/// Err implies an error during the generation or checking of the witness
/// Ok implies the witness was successfully generated and checked. The internal Result
/// indicates if the witness was successful, or if it failed.
///
/// ## Note: The unit types in the return type are temporary placeholders until the witness system is complete
fn run_single_witness_check(
    witness_set_dir: &Path,
    witness_name: &str,
    index: usize,
    witness_text: &str,
    dependency_data: &DependencyData,
) -> Result<Result<(), ()>> {
    let _crate_path = generate_witness_crate(
        witness_set_dir,
        witness_name,
        index,
        witness_text,
        dependency_data,
    )
    .with_context(|| format!("error generating witness crate `{witness_name}-{index}`"))?;

    // TODO: Check the witness crate that was generated

    Ok(Ok(()))
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
    let dependency_data = match DependencyData::parse_input(witness_data) {
        Ok(data) => data,
        Err(err) => {
            print_warning(
                config,
                format!(
                    "encountered non-fatal error while creating witness program \
                    {crate_name}: {err} (root cause: {})",
                    err.root_cause()
                ),
            );
            return;
        }
    };
    let witness_set_dir = dependency_data
        .target_dir
        .join(format!("{}-{crate_name}", config.run_id()));

    // Have to pull out handlebars, since &GlobalConfig cannot be shared across threads
    let handlebars = config.handlebars();

    lint_results.par_iter().for_each(|lint_result| {
        if let Some((semver_query, witness_texts)) = map_to_witness_text(
            handlebars,
            &lint_result.semver_query,
            &lint_result.query_results,
            adapter,
        ) {
            witness_texts
                .into_iter()
                .enumerate()
                .for_each(|(index, witness_text)| {
                    let _witness_result = witness_text.map(|witness_text| {
                        run_single_witness_check(
                            &witness_set_dir,
                            &semver_query.id,
                            index,
                            &witness_text,
                            &dependency_data,
                        )
                    });

                    // TODO: Save witness results and report them
                    // Must be reported outside of the par_iter since GlobalConfig cannot be shared across threads
                })
        }
    });
}
