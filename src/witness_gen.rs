mod lint_logic;
pub(crate) mod results;

use std::{
    collections::{BTreeMap, btree_map},
    fs,
    io::Write,
    ops::Deref,
    path::{Path, PathBuf},
    process::{ExitStatus, Stdio},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use handlebars::Handlebars;
use itertools::Either;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use trustfall::{FieldValue, TransparentValue};
use trustfall_rustdoc::VersionedRustdocAdapter;

use crate::{
    GlobalConfig, SemverQuery,
    query::{LintLogic, WitnessLogic},
    witness_gen::results::{WitnessChecksResultKind, WitnessLogicKinds},
};
use crate::{check_release::LintResult, data_generation::RequestKind};
use crate::{
    data_generation::{CrateDataRequest, ProjectRequest, RegistryRequest},
    query::{Witness, WitnessQuery},
};

use results::{WitnessCheckResult, WitnessCheckStatus};

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

pub(crate) struct WitnessRustdocPaths {
    baseline: PathBuf,
    current: PathBuf,
}

impl WitnessRustdocPaths {
    pub(crate) fn new(baseline: PathBuf, current: PathBuf) -> Self {
        Self { baseline, current }
    }
}

pub(crate) struct CoupledRustdocPath<T> {
    value: T,
    path: PathBuf,
}

impl<T> Deref for CoupledRustdocPath<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> CoupledRustdocPath<T> {
    pub fn new(value: T, path: PathBuf) -> Self {
        Self { value, path }
    }

    pub fn decouple(self) -> (T, PathBuf) {
        (self.value, self.path)
    }
}

/// Diagnostic information about a single failed or errored witness check
#[derive(Debug)]
pub(crate) struct SingleWitnessCheckInfo {
    pub witness_name: String,
    pub index: usize,
    // TODO: Add more collected diagnostic information
}

/// Diagnostics information with exit statuses attached
#[derive(Debug)]
pub(crate) struct SingleWitnessCheckExtraInfo {
    pub info: SingleWitnessCheckInfo,
    pub baseline_status: ExitStatus,
    pub current_status: ExitStatus,
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

#[derive(Debug)]
pub(crate) struct WitnessReport {
    /// Path in which the witnesses were generated
    pub witness_set_dir: PathBuf,
    /// Total amount of time to run witness checks
    pub time_elapsed: Duration,
    /// Total number of lints with witness checks
    pub lints_with_checks: usize,
    /// Total number of witnesses generated and run
    pub generated_checks: usize,
    /// Total number of errored witnesses
    pub errored_checks: usize,
    /// Total number of failed checks
    pub failed_checks: usize,
    /// Total number of failed checks which were repurposed for informing lint behaviour
    pub repurposed_failed_checks: usize,
    /// Total number of succeeded witnesses
    pub succeeded_checks: usize,
}

struct WitnessReportBuilder {
    witness_set_dir: PathBuf,
    start_instant: Instant,
    lints_with_checks: AtomicUsize,
    generated_checks: AtomicUsize,
    errored_checks: AtomicUsize,
    failed_checks: AtomicUsize,
    repurposed_failed_checks: AtomicUsize,
    succeeded_checks: AtomicUsize,
}

impl WitnessReport {
    pub fn failed(&self) -> bool {
        self.errored_checks > 0 || self.failed_checks > 0
    }
}

impl WitnessReportBuilder {
    fn new(witness_set_dir: impl Into<PathBuf>) -> Self {
        Self {
            witness_set_dir: witness_set_dir.into(),
            start_instant: Instant::now(),
            lints_with_checks: AtomicUsize::new(0),
            generated_checks: AtomicUsize::new(0),
            errored_checks: AtomicUsize::new(0),
            failed_checks: AtomicUsize::new(0),
            repurposed_failed_checks: AtomicUsize::new(0),
            succeeded_checks: AtomicUsize::new(0),
        }
    }

    fn lint_checked(&self) {
        self.lints_with_checks.fetch_add(1, Ordering::Relaxed);
    }

    fn ran_check(&self, status: &Result<WitnessCheckStatus>, logic: &LintLogic) {
        self.generated_checks.fetch_add(1, Ordering::Relaxed);
        match status {
            Ok(WitnessCheckStatus::BreakingChange) => {
                self.succeeded_checks.fetch_add(1, Ordering::Relaxed);
            }
            Ok(WitnessCheckStatus::NoBreakingChange { .. }) => {
                if logic.is_standard() {
                    self.failed_checks.fetch_add(1, Ordering::Relaxed);
                } else {
                    self.repurposed_failed_checks
                        .fetch_add(1, Ordering::Relaxed);
                }
            }
            Ok(WitnessCheckStatus::ErroredCase { .. }) | Err(_) => {
                self.errored_checks.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    fn finalize(self) -> WitnessReport {
        WitnessReport {
            witness_set_dir: self.witness_set_dir,
            time_elapsed: self.start_instant.elapsed(),
            lints_with_checks: self.lints_with_checks.into_inner(),
            generated_checks: self.generated_checks.into_inner(),
            errored_checks: self.errored_checks.into_inner(),
            failed_checks: self.failed_checks.into_inner(),
            repurposed_failed_checks: self.repurposed_failed_checks.into_inner(),
            succeeded_checks: self.succeeded_checks.into_inner(),
        }
    }
}

enum WitnessTextResults {
    StandardLogic(Vec<Result<String>>),
    WitnessLogic(WitnessLogicTextResults),
}

enum WitnessLogicTextResults {
    // Only one variant is present of right now, though this is meant as a future-proof system to allow
    // for new types of witness logic
    ExtractFuncArgs(WitnessLogicTextResult<BTreeMap<Arc<str>, FieldValue>>),
}

type WitnessLogicTextResult<T> = Vec<Result<(String, T)>>;

impl WitnessLogicTextResults {
    fn is_empty(&self) -> bool {
        match self {
            Self::ExtractFuncArgs(text_and_values) => text_and_values.is_empty(),
        }
    }
}

impl WitnessTextResults {
    fn is_empty(&self) -> bool {
        match self {
            Self::StandardLogic(texts) => texts.is_empty(),
            Self::WitnessLogic(witness_logic_result) => witness_logic_result.is_empty(),
        }
    }

    fn run_witness_checks(
        self,
        witness_set_dir: &Path,
        dependency_data: &DependencyData,
        report_builder: &WitnessReportBuilder,
        semver_query: &SemverQuery,
    ) -> Option<WitnessCheckResult> {
        if self.is_empty() {
            return None;
        }

        let mut breaking_change_found = false;
        let mut errored_check = false;

        // Use a closure here instead of a function in order to capture the large number of surrounding variables.
        // This is done purely due to the large potential number of [`WitnessLogic`] variants. As more systems
        // are added, we need to implement system-specific logic, so having a ton of consistent values captured
        // by a closure is a lot faster to read and write than a function that takes those values directly
        let mut check_witness = |index: usize, witness_text: String| {
            let witness_result = run_single_witness_check(
                witness_set_dir,
                &semver_query.id,
                index,
                &witness_text,
                dependency_data,
            );

            report_builder.ran_check(&witness_result, &semver_query.lint_logic);

            // Check for breaking change, with short circuit on if one has already been found
            if !breaking_change_found
                && witness_result
                    .as_ref()
                    .is_ok_and(|result| result.is_breaking())
            {
                breaking_change_found = true;
            }

            witness_result
        };

        let check_results = match self {
            Self::StandardLogic(texts) => {
                let mut results = vec![];
                texts
                    .into_iter()
                    .enumerate()
                    .for_each(|(index, text_result)| {
                        let witness_result =
                            text_result.and_then(|text| check_witness(index, text));

                        // Check for an error, with a short circuit on if one has already been found
                        if !errored_check
                            && (witness_result.is_err()
                                || witness_result
                                    .as_ref()
                                    .is_ok_and(|status| status.errored() || !status.is_breaking()))
                        {
                            errored_check = true;
                        }

                        results.push(witness_result);
                    });
                WitnessChecksResultKind::Standard(results)
            }
            Self::WitnessLogic(WitnessLogicTextResults::ExtractFuncArgs(texts_and_values)) => {
                let mut results = vec![];
                texts_and_values
                    .into_iter()
                    .enumerate()
                    .for_each(|(index, text_value_result)| {
                        let witness_result = text_value_result.and_then(|(text, extracted)| {
                            check_witness(index, text).map(|result| (result, extracted))
                        });

                        // Check for an error, with a short circuit on if one has already been found
                        if !errored_check && witness_result.is_err()
                            || witness_result
                                .as_ref()
                                .is_ok_and(|(status, _)| status.errored())
                        {
                            errored_check = true;
                        }

                        results.push(witness_result);
                    });
                WitnessChecksResultKind::WitnessLogic(WitnessLogicKinds::ExtractFuncArgs(results))
            }
        };

        Some(WitnessCheckResult {
            check_results,
            has_errored_check: errored_check,
            breaking_change_found,
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

fn run_extra_logic_and_textgen(
    handlebars: &Handlebars,
    handlebars_template: &str,
    queried_values: impl Iterator<Item = Result<BTreeMap<Arc<str>, FieldValue>>>,
    semver_query: &SemverQuery,
    rustdoc_paths: &WitnessRustdocPaths,
) -> WitnessTextResults {
    match semver_query.lint_logic {
        LintLogic::UseStandard => {
            let texts = queried_values
                .map(|result| {
                    result.and_then(|values| {
                        generate_witness_text(handlebars, handlebars_template, values)
                    })
                })
                .collect();
            WitnessTextResults::StandardLogic(texts)
        }
        LintLogic::UseWitness(WitnessLogic::ExtractFuncArgs) => {
            let texts_and_values = queried_values
                .map(|result| {
                    result
                        .and_then(|values| lint_logic::extract_func_args(values, rustdoc_paths))
                        .and_then(|values| {
                            Ok((
                                generate_witness_text(
                                    handlebars,
                                    handlebars_template,
                                    values.clone(),
                                )?,
                                values,
                            ))
                        })
                })
                .collect();
            WitnessTextResults::WitnessLogic(WitnessLogicTextResults::ExtractFuncArgs(
                texts_and_values,
            ))
        }
    }
}

#[expect(clippy::complexity)]
fn map_to_witness_text<'query>(
    handlebars: &Handlebars,
    semver_query: &'query SemverQuery,
    lint_results: &[BTreeMap<Arc<str>, FieldValue>],
    adapter: &VersionedRustdocAdapter,
    rustdoc_paths: &WitnessRustdocPaths,
) -> Option<WitnessTextResults> {
    let (template, iter) = match &semver_query.witness {
        // Don't bother running the witness query unless both a witness query and template exist
        Some(Witness {
            witness_template: Some(witness_template),
            witness_query: Some(witness_query),
            ..
        }) => (
            witness_template,
            Either::Left(lint_results.iter().cloned().map(|lint_result| {
                run_witness_query(adapter, witness_query, lint_result)
                    .with_context(|| format!("error running witness query for {}", semver_query.id))
            })),
        ),

        // If no witness query exists, we still want to forward the existing output
        Some(Witness {
            witness_template: Some(witness_template),
            witness_query: None,
            ..
        }) => (
            witness_template,
            Either::Right(lint_results.iter().cloned().map(Ok)),
        ),
        _ => return None,
    };

    let text_results =
        run_extra_logic_and_textgen(handlebars, template, iter, semver_query, rustdoc_paths);

    Some(text_results)
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

    fs::create_dir_all(&src_path).with_context(|| {
        format!(
            "failed to create all directories in the path {}",
            src_path.display()
        )
    })?;

    let mut manifest_file = fs::File::create(crate_path.join("Cargo.toml")).with_context(|| {
        format!(
            "failed to create witness crate cargo manifest {}",
            crate_path.join("Cargo.toml").display()
        )
    })?;

    let mut lib_file = fs::File::create(src_path.join("lib.rs")).with_context(|| {
        format!(
            "failed to create main witness file {}",
            src_path.join("lib.rs").display()
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

fn cargo_check_witness<F>(crate_path: &Path, build_diagnostics: F) -> Result<WitnessCheckStatus>
where
    F: Fn() -> SingleWitnessCheckInfo,
{
    // FIXME: Update from static "cargo" to dynamic system in case cargo is not in $PATH or is not called `cargo`
    let exe = "cargo";

    let crate_path = crate_path.join("Cargo.toml");

    let baseline_status = std::process::Command::new(exe)
        .args([
            "check",
            "--quiet",
            "--manifest-path",
            &crate_path.display().to_string(),
            "--features",
            "baseline",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .with_context(|| {
            format!(
                "error checking witness crate with baseline at {}",
                crate_path.display()
            )
        })?;

    let current_status = std::process::Command::new(exe)
        .args([
            "check",
            "--quiet",
            "--manifest-path",
            &crate_path.display().to_string(),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .with_context(|| {
            format!(
                "error checking witness crate with current at {}",
                crate_path.display()
            )
        })?;

    let check_status = match (baseline_status.success(), current_status.success()) {
        (true, false) => WitnessCheckStatus::BreakingChange,
        (true, true) => WitnessCheckStatus::NoBreakingChange(build_diagnostics()),
        (false, _) => WitnessCheckStatus::ErroredCase(SingleWitnessCheckExtraInfo {
            info: build_diagnostics(),
            baseline_status,
            current_status,
        }),
    };

    Ok(check_status)
}

/// Runs a single witness check, returning the result of the witness process
///
/// Err implies an error during the generation or checking of the witness
/// Ok implies the witness was successfully generated and checked. The internal Result
/// indicates if the witness was successful, or if it failed.
fn run_single_witness_check(
    witness_set_dir: &Path,
    witness_name: &str,
    index: usize,
    witness_text: &str,
    dependency_data: &DependencyData,
) -> Result<WitnessCheckStatus> {
    let crate_path = generate_witness_crate(
        witness_set_dir,
        witness_name,
        index,
        witness_text,
        dependency_data,
    )
    .with_context(|| format!("error generating witness crate `{witness_name}-{index}`"))?;

    let check_status = cargo_check_witness(
        &crate_path,
        // Currently the diagnostics are initialized here. If additional diagnostic information is added,
        // consider hoisting initialization elsewhere.
        || SingleWitnessCheckInfo {
            witness_name: witness_name.to_string(),
            index,
        },
    )?;

    Ok(check_status)
}

/// Run all witness checks, returning a [`WitnessReport`], which contains details about the witnesses that were run.
///
/// All [`WitnessResult`]s are added in place to `lint_results` retroactively, overriding [`LintResult::witness_results`].
/// Any [`LintResult`] with a `witness_result` of [`Some`] will contain at least one value.
pub(crate) fn run_witness_checks(
    config: &mut GlobalConfig,
    witness_data: WitnessGenerationData,
    rustdoc_paths: WitnessRustdocPaths,
    crate_name: &str,
    adapter: &VersionedRustdocAdapter,
    lint_results: &mut [LintResult],
) -> Result<WitnessReport> {
    let dependency_data = DependencyData::parse_input(witness_data).with_context(|| {
        format!("failure creating witness, could not parse input for crate {crate_name}")
    })?;

    let witness_set_dir = dependency_data
        .target_dir
        .join(format!("{}-{crate_name}", config.run_id()));

    // Have to pull out handlebars, since &GlobalConfig cannot be shared across threads
    let handlebars = config.handlebars();

    let report = WitnessReportBuilder::new(&witness_set_dir);
    lint_results.par_iter_mut().for_each(|lint_result| {
        let witness_check_result = map_to_witness_text(
            handlebars,
            &lint_result.semver_query,
            &lint_result.query_results,
            adapter,
            &rustdoc_paths,
        )
        .and_then(|text_result| {
            text_result.run_witness_checks(
                &witness_set_dir,
                &dependency_data,
                &report,
                &lint_result.semver_query,
            )
        });
        if let Some(witness_check_result) = witness_check_result {
            lint_result.witness_results = Some(witness_check_result);
            report.lint_checked();
        }
    });

    Ok(report.finalize())
}
