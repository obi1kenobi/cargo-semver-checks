use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::io::Write as _;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anstyle::{AnsiColor, Color, Reset, Style};

use anyhow::Context;
use clap::crate_version;
use itertools::Itertools;
use rayon::prelude::*;
use trustfall::{FieldValue, TransparentValue};

use crate::data_generation::DataStorage;
use crate::query::{
    ActualSemverUpdate, LintLevel, OverrideStack, RequiredSemverUpdate, SemverQuery,
};
use crate::witness_gen;
use crate::{Bumps, CrateReport, GlobalConfig, ReleaseType, WitnessGeneration};

/// Represents a change between two semantic versions
#[derive(Debug, PartialEq, Eq)]
struct VersionChange {
    /// The level of the semantic version update (major, minor, patch)
    level: ActualSemverUpdate,
    /// Whether this is an actual change between different versions
    /// or a minimum possible change for the same version
    kind: VersionChangeKind,
}

#[derive(Debug, PartialEq, Eq)]
enum VersionChangeKind {
    Actual,
    Minimum,
}

/// Classifies the minimum semantic version change between two versions.
/// It would panic if either version is not a valid semver.
fn classify_minimum_semver_version_change(
    current_version: &str,
    baseline_version: &str,
) -> VersionChange {
    let baseline_version =
        semver::Version::parse(baseline_version).expect("baseline not a valid version");
    let current_version =
        semver::Version::parse(current_version).expect("current not a valid version");

    // Check if versions are identical (ignoring build metadata)
    if baseline_version.cmp_precedence(&current_version) == Ordering::Equal {
        if !baseline_version.pre.is_empty() {
            // the baseline is a pre-release: the minimum "next" change is major
            return VersionChange {
                level: ActualSemverUpdate::Major,
                kind: VersionChangeKind::Minimum,
            };
        }
        return get_minimum_version_change(&current_version);
    }

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
    } else if baseline_version.pre != current_version.pre {
        // > A pre-release version indicates that the version is unstable and might not satisfy
        // > the intended compatibility requirements as denoted by its associated normal version
        // https://semver.org/#spec-item-9
        ActualSemverUpdate::Major
    } else {
        unreachable!(
            "versions have identical major.minor.patch components, but did not \
                register as equal when compared: \
                {current_version:?} vs {baseline_version:?}"
        );
    };

    VersionChange {
        level: update_kind,
        kind: VersionChangeKind::Actual,
    }
}

fn get_minimum_version_change(version: &semver::Version) -> VersionChange {
    let update = match (version.major, version.minor) {
        // For 0.0.z: Minimum next change must be major
        (0, 0) => ActualSemverUpdate::Major,
        // For 0.y.z: Minimum next change must be minor
        (0, _) => ActualSemverUpdate::Minor,
        // For x.y.z: Minimum next change must be patch
        (_, _) => ActualSemverUpdate::Patch,
    };

    VersionChange {
        level: update,
        kind: VersionChangeKind::Minimum,
    }
}

/// Intermediate state in [`run_check_release`]
#[derive(Debug)]
pub(crate) struct LintResult {
    pub semver_query: SemverQuery,
    pub query_results: Vec<BTreeMap<Arc<str>, FieldValue>>,
    /// How long it took to run the semver query
    pub query_duration: Duration,
    /// Applied `OverrideStack`
    pub effective_required_update: RequiredSemverUpdate,
    pub effective_lint_level: LintLevel,
}

/// Internal per-crate check result that is still pending post-processing
/// before it can become user-facing data.
///
/// [`run_check_release()`] produces this type to hold the user-facing [`CrateReport`]
/// together with witness bookkeeping that must be finalized only after
/// all crates in the run have been checked.
#[derive(Debug)]
pub(crate) struct PendingCrateReport {
    /// User-facing semver report after witness evaluation has already rewritten
    /// the authoritative lint result set.
    pub(crate) report: CrateReport,
    /// Internal witness bookkeeping that still needs to be finalized into
    /// retained run artifacts and manifests after all crates have been checked.
    pub(crate) witness_run_report: witness_gen::WitnessRunReport,
}

/// Helper function to print details about a triggered lint.
fn print_triggered_lint(
    config: &mut GlobalConfig,
    lint_result: &LintResult,
    witness_generation: &WitnessGeneration,
) -> anyhow::Result<()> {
    let semver_query = &lint_result.semver_query;
    let lint_level = lint_result.effective_lint_level;

    if let Some(ref_link) = semver_query.reference_link.as_deref() {
        config.log_at_lint_level(lint_level, |config| {
            writeln!(config.stdout(), "{}Description:{}\n{}\n{:>12} {}\n{:>12} https://github.com/obi1kenobi/cargo-semver-checks/tree/v{}/src/lints/{}.ron\n",
                Style::new().bold(), Reset,
                &semver_query.error_message,
                "ref:",
                ref_link,
                "impl:",
                crate_version!(),
                semver_query.id,
            )?;
            Ok(())
        })?;
    } else {
        config.log_at_lint_level(lint_level, |config| {
            writeln!(
                config.stdout(),
                "{}Description:{}\n{}\n{:>12} https://github.com/obi1kenobi/cargo-semver-checks/tree/v{}/src/lints/{}.ron",
                Style::new().bold(),
                Reset,
                &semver_query.error_message,
                "impl:",
                crate_version!(),
                semver_query.id,
            )?;
            Ok(())
        })?;
    }

    config.log_at_lint_level(lint_level, |config| {
        writeln!(
            config.stdout(),
            "{}Failed in:{}",
            Style::new().bold(),
            Reset
        )?;
        Ok(())
    })?;

    for semver_violation_result in &lint_result.query_results {
        let pretty_result: BTreeMap<&str, TransparentValue> = semver_violation_result
            .iter()
            .map(|(k, v)| (&**k, v.clone().into()))
            .collect();

        if let Some(template) = semver_query.per_result_error_template.as_deref() {
            let message = config
                .handlebars()
                .render_template(template, &pretty_result)
                .context("Error instantiating semver query template.")
                .expect("could not materialize template");
            config.log_at_lint_level(lint_level, |config| {
                writeln!(config.stdout(), "  {message}")?;
                Ok(())
            })?;

            config.log_extra_verbose(|config| {
                let serde_pretty =
                    serde_json::to_string_pretty(&pretty_result).expect("serde failed");
                let indented_serde = serde_pretty
                    .split('\n')
                    .map(|line| format!("    {line}"))
                    .join("\n");
                writeln!(
                    config.stdout(),
                    "\tlint rule output values:\n{indented_serde}"
                )?;
                Ok(())
            })?;
        } else {
            config.log_at_lint_level(lint_level, |config| {
                writeln!(
                    config.stdout(),
                    "{}\n",
                    serde_json::to_string_pretty(&pretty_result)?
                )?;
                Ok(())
            })?;
        }

        if let Some(witness) = &semver_query.witness
            && witness_generation.show_hints
        {
            let message = config
                .handlebars()
                .render_template(&witness.hint_template, &pretty_result)
                .context("Error instantiating witness hint template.")?;

            config.log_at_lint_level(lint_level, |config| {
                let note = Style::new()
                    .fg_color(Some(Color::Ansi(AnsiColor::Cyan)))
                    .bold();
                writeln!(
                    config.stdout(),
                    "{note}note:{note:#} downstream code similar to the following would break:\n\
                        {message}\n"
                )?;
                Ok(())
            })?;
        }
    }

    Ok(())
}

pub(super) fn run_check_release(
    config: &mut GlobalConfig,
    data_storage: &DataStorage,
    crate_name: &str,
    release_type: Option<ReleaseType>,
    overrides: &OverrideStack,
    witness_generation: &WitnessGeneration,
    witness_data: witness_gen::WitnessGenerationData,
) -> anyhow::Result<PendingCrateReport> {
    let current_version = data_storage.current_crate().crate_version();
    let baseline_version = data_storage.baseline_crate().crate_version();

    let version_change = match release_type {
        // Case 1: User explicitly specified a release type
        Some(rt) => VersionChange {
            level: rt.into(),
            kind: VersionChangeKind::Actual,
        },
        // Case 2: Try to determine from version strings
        None => match (baseline_version, current_version) {
            (Some(baseline), Some(current)) => {
                classify_minimum_semver_version_change(baseline, current)
            }
            // Case 3: Fall back to assuming no change
            _ => {
                config
                    .shell_warn(
                        "Could not determine whether crate version changed. Assuming no change.",
                    )
                    .expect("print failed");
                VersionChange {
                    level: ActualSemverUpdate::NotChanged,
                    kind: VersionChangeKind::Actual,
                }
            }
        },
    };
    let change = match version_change.level {
        ActualSemverUpdate::Major => "major",
        ActualSemverUpdate::Minor => "minor",
        ActualSemverUpdate::Patch => "patch",
        ActualSemverUpdate::NotChanged => "no",
    };
    let assume = if release_type.is_some() || version_change.kind == VersionChangeKind::Minimum {
        "assume "
    } else {
        ""
    };

    let change_message = match version_change.kind {
        VersionChangeKind::Actual => format!("{assume}{change} change"),
        VersionChangeKind::Minimum => format!("no change; {assume}{change}"),
    };

    let index_storage = data_storage.create_indexes();
    let adapter = index_storage.create_adapter();

    let mut queries_to_run = SemverQuery::all_queries();
    let all_queries_len = queries_to_run.len();
    queries_to_run.retain(|_, query| {
        !version_change
            .level
            .supports_requirement(overrides.effective_required_update(query))
            && overrides.effective_lint_level(query) > LintLevel::Allow
    });
    let selected_checks = queries_to_run.len();
    let skipped_checks = all_queries_len - selected_checks;

    config.shell_status(
        "Checking",
        format_args!(
            "{crate_name} v{} -> v{} ({})",
            baseline_version.unwrap_or("unknown"),
            current_version.unwrap_or("unknown"),
            change_message
        ),
    )?;
    config
        .log_verbose(|config| {
            let current_num_threads = rayon::current_num_threads();
            if current_num_threads == 1 {
                config.shell_status(
                    "Starting",
                    format_args!("{} checks, {} unnecessary", selected_checks, skipped_checks),
                )
            } else {
                config.shell_status(
                    "Starting",
                    format_args!(
                        "{} checks, {} unnecessary on {current_num_threads} threads",
                        selected_checks, skipped_checks
                    ),
                )
            }
        })
        .expect("print failed");

    let checks_start_instant = Instant::now();
    let mut lint_results = queries_to_run
        .into_par_iter()
        .map(|(_, semver_query)| {
            let start_instant = std::time::Instant::now();
            // trustfall::execute_query(...) -> dyn Iterator (without Send)
            // thus the result must be collect()'ed
            let query_results = adapter
                .run_query(&semver_query.query, semver_query.arguments.clone())?
                .collect_vec();
            let query_duration = start_instant.elapsed();
            Ok(LintResult {
                effective_required_update: overrides.effective_required_update(&semver_query),
                effective_lint_level: overrides.effective_lint_level(&semver_query),
                semver_query,
                query_duration,
                query_results,
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let witness_run_report = witness_gen::run_witness_checks(
        config,
        witness_generation,
        witness_data,
        crate_name,
        &adapter,
        &mut lint_results,
    );

    let checks_duration = checks_start_instant.elapsed();

    let mut required_bumps = Bumps { major: 0, minor: 0 };
    let mut suggested_bumps = Bumps { major: 0, minor: 0 };
    for result in &lint_results {
        if !result.query_results.is_empty() {
            let bump_stats = match result.effective_lint_level {
                LintLevel::Deny => &mut required_bumps,
                LintLevel::Warn => &mut suggested_bumps,
                LintLevel::Allow => unreachable!(
                    "`LintLevel::Allow` lint was unexpectedly not skipped: {:?}",
                    result.semver_query
                ),
            };
            match result.effective_required_update {
                RequiredSemverUpdate::Major => bump_stats.major += 1,
                RequiredSemverUpdate::Minor => bump_stats.minor += 1,
            };
        }
    }

    let report = CrateReport {
        lint_results,
        checks_duration,
        selected_checks,
        skipped_checks,
        required_bumps,
        suggested_bumps,
        detected_bump: version_change.level,
        witness_statistics: witness_run_report.statistics.clone(),
    };

    print_report(config, witness_generation, &report)?;
    Ok(PendingCrateReport {
        report,
        witness_run_report,
    })
}

fn print_report(
    config: &mut GlobalConfig,
    witness_generation: &WitnessGeneration,
    report: &CrateReport,
) -> anyhow::Result<()> {
    let mut results_with_errors = vec![];
    let mut results_with_warnings = vec![];
    let witness_statistics = report.witness_statistics.as_ref();
    let produced_required_witness_errors =
        witness_statistics.is_some_and(|statistics| statistics.required_witness_errors() > 0);
    let produced_witness_warnings = witness_statistics.is_some_and(|statistics| {
        statistics.consistency_check_mismatches() > 0 || statistics.consistency_check_errors() > 0
    });

    for result in &report.lint_results {
        config
            .log_verbose(|config| {
                let category = match result.effective_required_update {
                    RequiredSemverUpdate::Major => "major",
                    RequiredSemverUpdate::Minor => "minor",
                };

                let (status, status_color) =
                    match (result.query_results.is_empty(), result.effective_lint_level) {
                        (true, _) => ("PASS", AnsiColor::Green),
                        (false, LintLevel::Deny) => ("FAIL", AnsiColor::Red),
                        (false, LintLevel::Warn) => ("WARN", AnsiColor::Yellow),
                        (false, LintLevel::Allow) => unreachable!(
                            "`LintLevel::Allow` lint was unexpectedly not skipped: {:?}",
                            result.semver_query
                        ),
                    };

                writeln!(
                    config.stderr(),
                    "{}{:>12}{} [{:8.3}s] {:^18} {}",
                    Style::new()
                        .fg_color(Some(Color::Ansi(status_color)))
                        .bold(),
                    status,
                    Reset,
                    result.query_duration.as_secs_f32(),
                    category,
                    result.semver_query.id
                )?;
                Ok(())
            })
            .expect("print failed");

        if !result.query_results.is_empty() {
            match result.effective_lint_level {
                LintLevel::Deny => results_with_errors.push(result),
                LintLevel::Warn => results_with_warnings.push(result),
                LintLevel::Allow => unreachable!(
                    "`LintLevel::Allow` lint was unexpectedly not skipped: {:?}",
                    result.semver_query
                ),
            };
        }
    }

    let produced_errors = !results_with_errors.is_empty();
    let produced_warnings = !results_with_warnings.is_empty();
    if produced_errors
        || produced_warnings
        || produced_required_witness_errors
        || produced_witness_warnings
    {
        let status_color = if produced_errors || produced_required_witness_errors {
            AnsiColor::Red
        } else {
            AnsiColor::Yellow
        };
        config
            .shell_print(
                "Checked",
                format_args!(
                    "[{:>8.3}s] {} checks: {} pass, {} fail, {} warn, {} skip",
                    report.checks_duration.as_secs_f32(),
                    report.selected_checks,
                    report.selected_checks
                        - results_with_errors.len()
                        - results_with_warnings.len(),
                    results_with_errors.len(),
                    results_with_warnings.len(),
                    report.skipped_checks,
                ),
                Color::Ansi(status_color),
                true,
            )
            .expect("print failed");

        for lint_result in results_with_errors {
            config.log_error(|config| {
                writeln!(
                    config.stdout(),
                    "\n--- failure {}: {} ---\n",
                    lint_result.semver_query.id,
                    lint_result.semver_query.human_readable_name
                )?;
                Ok(())
            })?;

            print_triggered_lint(config, lint_result, witness_generation)?;
        }

        for lint_result in results_with_warnings {
            config.log_warn(|config| {
                writeln!(
                    config.stdout(),
                    "\n--- warning {}: {} ---\n",
                    lint_result.semver_query.id,
                    lint_result.semver_query.human_readable_name
                )?;
                Ok(())
            })?;

            print_triggered_lint(config, lint_result, witness_generation)?;
        }

        if let Some(required_bump) = report.required_bumps.update_type() {
            writeln!(config.stderr())?;
            config.shell_print(
                "Summary",
                format_args!(
                    "semver requires new {} version: {} major and {} minor checks failed",
                    required_bump.as_str(),
                    report.required_bumps.major,
                    report.required_bumps.minor,
                ),
                Color::Ansi(AnsiColor::Red),
                true,
            )?;
        } else if produced_required_witness_errors {
            writeln!(config.stderr())?;
            let required_witness_errors = witness_statistics
                .expect("required witness error summary requires witness statistics")
                .required_witness_errors();
            config.shell_print(
                "Summary",
                format_args!(
                    "required witness validation failed: encountered \
                     {required_witness_errors} required witness error{}",
                    if required_witness_errors == 1 {
                        ""
                    } else {
                        "s"
                    }
                ),
                Color::Ansi(AnsiColor::Red),
                true,
            )?;
        } else {
            writeln!(config.stderr())?;
            config.shell_print(
                "Summary",
                "no semver update required",
                Color::Ansi(AnsiColor::Green),
                true,
            )?;
        }

        if let Some(suggested_bump) = report.suggested_bumps.update_type() {
            config.shell_print(
                "Warning",
                format_args!(
                    "produced {} major and {} minor level warnings",
                    report.suggested_bumps.major, report.suggested_bumps.minor,
                ),
                Color::Ansi(AnsiColor::Yellow),
                true,
            )?;

            if report
                .required_bumps
                .update_type()
                .is_none_or(|required_bump| required_bump < suggested_bump)
            {
                writeln!(
                    config.stderr(),
                    "{:12} produced warnings suggest new {} version",
                    "",
                    suggested_bump.as_str(),
                )?;
            }
        }
    } else {
        config
            .shell_print(
                "Checked",
                format_args!(
                    "[{:>8.3}s] {} checks: {} pass, {} skip",
                    report.checks_duration.as_secs_f32(),
                    report.selected_checks,
                    report.selected_checks,
                    report.skipped_checks,
                ),
                Color::Ansi(AnsiColor::Green),
                true,
            )
            .expect("print failed");

        config.shell_print(
            "Summary",
            "no semver update required",
            Color::Ansi(AnsiColor::Green),
            true,
        )?;
    }

    if let Some(statistics) = witness_statistics {
        if statistics.not_confirmed_by_witness() > 0 {
            config.shell_note(format_args!(
                "suppressed {} query results that were not confirmed by witness",
                statistics.not_confirmed_by_witness()
            ))?;
        }

        if statistics.consistency_check_mismatches() > 0
            || statistics.consistency_check_errors() > 0
        {
            config.shell_warn(format_args!(
                "consistency checks found {} results not confirmed by witness \
                 and {} witness errors",
                statistics.consistency_check_mismatches(),
                statistics.consistency_check_errors(),
            ))?;
        }

        if statistics.required_witness_errors() > 0 {
            config.shell_error(format_args!(
                "encountered {} required witness errors",
                statistics.required_witness_errors(),
            ))?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::{
        cell::RefCell,
        io::{Cursor, Read as _, Seek as _, Write},
        rc::Rc,
    };

    use super::*;

    #[derive(Clone)]
    struct SharedBuffer(Rc<RefCell<Cursor<Vec<u8>>>>);

    impl SharedBuffer {
        fn new() -> Self {
            Self(Rc::new(RefCell::new(Cursor::new(Vec::new()))))
        }

        fn contents(&self) -> String {
            let mut cursor = self.0.borrow_mut();
            cursor.rewind().expect("failed to rewind shared buffer");
            let mut output = String::new();
            cursor
                .read_to_string(&mut output)
                .expect("failed to read shared buffer contents");
            output
        }
    }

    impl Write for SharedBuffer {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.borrow_mut().write(buf)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.0.borrow_mut().flush()
        }
    }

    #[test]
    fn classify_same_version() {
        let baseline = "1.0.0";
        let current = "1.0.0";
        let expected = VersionChange {
            level: ActualSemverUpdate::Patch,
            kind: VersionChangeKind::Minimum,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_patch_changed() {
        let baseline = "1.0.0";
        let current = "1.0.1";
        let expected = VersionChange {
            level: ActualSemverUpdate::Patch,
            kind: VersionChangeKind::Actual,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_minor_changed() {
        let baseline = "1.0.0";
        let current = "1.1.0";
        let expected = VersionChange {
            level: ActualSemverUpdate::Minor,
            kind: VersionChangeKind::Actual,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_major_changed() {
        let baseline = "0.9.0";
        let current = "1.0.0";
        let expected = VersionChange {
            level: ActualSemverUpdate::Major,
            kind: VersionChangeKind::Actual,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_zerover_minor_changed() {
        let baseline = "0.1.0";
        let current = "0.1.1";
        let expected = VersionChange {
            level: ActualSemverUpdate::Minor,
            kind: VersionChangeKind::Actual,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_zerover_major_changed() {
        let baseline = "0.1.0";
        let current = "0.2.0";
        let expected = VersionChange {
            level: ActualSemverUpdate::Major,
            kind: VersionChangeKind::Actual,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_double_zerover_major_changed() {
        let baseline = "0.0.1";
        let current = "0.0.2";
        let expected = VersionChange {
            level: ActualSemverUpdate::Major,
            kind: VersionChangeKind::Actual,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_pre_same() {
        let baseline = "1.0.0-alpha.0";
        let current = "1.0.0-alpha.0";
        let expected = VersionChange {
            level: ActualSemverUpdate::Major,
            kind: VersionChangeKind::Minimum,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_pre() {
        let baseline = "1.0.0-alpha.0";
        let current = "1.0.0-alpha.1";
        let expected = VersionChange {
            level: ActualSemverUpdate::Major,
            kind: VersionChangeKind::Actual,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_same_version_with_pre() {
        let baseline = "1.0.0-alpha.1";
        let current = "1.0.0";
        let expected = VersionChange {
            level: ActualSemverUpdate::Major,
            kind: VersionChangeKind::Actual,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_minor_changed_with_pre() {
        let baseline = "1.0.0";
        let current = "1.1.0-alpha.1";
        let expected = VersionChange {
            level: ActualSemverUpdate::Minor,
            kind: VersionChangeKind::Actual,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_zerover_same_version() {
        let baseline = "0.1.0";
        let current = "0.1.0";
        let expected = VersionChange {
            level: ActualSemverUpdate::Minor,
            kind: VersionChangeKind::Minimum,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_zerover_zero_same_version() {
        let baseline = "0.0.1";
        let current = "0.0.1";
        let expected = VersionChange {
            level: ActualSemverUpdate::Major,
            kind: VersionChangeKind::Minimum,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_pre_zero_same() {
        let baseline = "0.1.0-alpha.0";
        let current = "0.1.0-alpha.0";
        let expected = VersionChange {
            level: ActualSemverUpdate::Major,
            kind: VersionChangeKind::Minimum,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_build_with_pre_same() {
        let baseline = "1.0.0-alpha.1+build.1";
        let current = "1.0.0-alpha.1+build.2";
        let expected = VersionChange {
            level: ActualSemverUpdate::Major,
            kind: VersionChangeKind::Minimum,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_build_zero_version() {
        let baseline = "0.1.0+build.1";
        let current = "0.1.0+build.2";
        let expected = VersionChange {
            level: ActualSemverUpdate::Minor,
            kind: VersionChangeKind::Minimum,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_ignores_build() {
        let baseline = "1.0.0+hello";
        let current = "1.0.0+world";
        let expected = VersionChange {
            level: ActualSemverUpdate::Patch,
            kind: VersionChangeKind::Minimum,
        };
        let actual = classify_minimum_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn crate_report_tracks_required_witness_errors_separately_from_semver_success() {
        let report = CrateReport {
            detected_bump: ActualSemverUpdate::NotChanged,
            required_bumps: Bumps { major: 0, minor: 0 },
            suggested_bumps: Bumps { major: 0, minor: 0 },
            lint_results: Vec::new(),
            checks_duration: Duration::ZERO,
            selected_checks: 1,
            skipped_checks: 0,
            witness_statistics: Some(crate::WitnessStatistics::new(0, 0, 0, 1)),
        };

        assert!(report.success());
        assert!(report.has_required_witness_errors());
    }

    #[test]
    fn report_cli_success_includes_required_witness_errors() {
        let crate_report = CrateReport {
            detected_bump: ActualSemverUpdate::NotChanged,
            required_bumps: Bumps { major: 0, minor: 0 },
            suggested_bumps: Bumps { major: 0, minor: 0 },
            lint_results: Vec::new(),
            checks_duration: Duration::ZERO,
            selected_checks: 1,
            skipped_checks: 0,
            witness_statistics: Some(crate::WitnessStatistics::new(0, 0, 0, 1)),
        };
        let report = crate::Report {
            crate_reports: BTreeMap::from([("demo".to_owned(), crate_report)]),
        };

        assert!(report.success());
        assert!(!report.is_cli_success());
    }

    #[test]
    fn print_report_mentions_not_confirmed_by_witness() {
        let stdout = SharedBuffer::new();
        let stderr = SharedBuffer::new();
        let mut config = GlobalConfig::new();
        config.set_log_level(Some(log::Level::Info));
        config.set_stdout(Box::new(stdout.clone()));
        config.set_stderr(Box::new(stderr.clone()));

        let report = CrateReport {
            detected_bump: ActualSemverUpdate::NotChanged,
            required_bumps: Bumps { major: 0, minor: 0 },
            suggested_bumps: Bumps { major: 0, minor: 0 },
            lint_results: Vec::new(),
            checks_duration: Duration::ZERO,
            selected_checks: 1,
            skipped_checks: 0,
            witness_statistics: Some(crate::WitnessStatistics::new(1, 0, 0, 0)),
        };

        print_report(&mut config, &WitnessGeneration::new(), &report)
            .expect("failed to print report");

        assert!(stderr.contents().contains("not confirmed by witness"));
    }

    #[test]
    fn print_report_does_not_report_success_on_required_witness_errors() {
        let stdout = SharedBuffer::new();
        let stderr = SharedBuffer::new();
        let mut config = GlobalConfig::new();
        config.set_log_level(Some(log::Level::Info));
        config.set_stdout(Box::new(stdout));
        config.set_stderr(Box::new(stderr.clone()));

        let report = CrateReport {
            detected_bump: ActualSemverUpdate::NotChanged,
            required_bumps: Bumps { major: 0, minor: 0 },
            suggested_bumps: Bumps { major: 0, minor: 0 },
            lint_results: Vec::new(),
            checks_duration: Duration::ZERO,
            selected_checks: 1,
            skipped_checks: 0,
            witness_statistics: Some(crate::WitnessStatistics::new(0, 0, 0, 1)),
        };

        print_report(&mut config, &WitnessGeneration::new(), &report)
            .expect("failed to print report");

        let stderr = stderr.contents();
        assert!(!stderr.contains("no semver update required"));
        assert!(stderr.contains("required witness validation failed"));
    }
}
