use std::cmp::Ordering;
use std::io::Write as _;
use std::{collections::BTreeMap, sync::Arc, time::Instant};

use anstyle::{AnsiColor, Color, Reset, Style};

use anyhow::Context;
use clap::crate_version;
use itertools::Itertools;
use rayon::prelude::*;
use trustfall::{FieldValue, TransparentValue};

use crate::data_generation::DataStorage;
use crate::witness_gen;
use crate::Witness;
use crate::{
    query::{ActualSemverUpdate, LintLevel, OverrideStack, RequiredSemverUpdate, SemverQuery},
    CrateReport, GlobalConfig, ReleaseType, WitnessGeneration,
};

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

/// Helper function to print details about a triggered lint.
fn print_triggered_lint(
    config: &mut GlobalConfig,
    semver_query: &SemverQuery,
    results: Vec<BTreeMap<Arc<str>, FieldValue>>,
    witness_generation: &WitnessGeneration,
) -> anyhow::Result<()> {
    if let Some(ref_link) = semver_query.reference_link.as_deref() {
        config.log_info(|config| {
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
        config.log_info(|config| {
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

    config.log_info(|config| {
        writeln!(
            config.stdout(),
            "{}Failed in:{}",
            Style::new().bold(),
            Reset
        )?;
        Ok(())
    })?;

    for semver_violation_result in results {
        let pretty_result: BTreeMap<Arc<str>, TransparentValue> = semver_violation_result
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect();

        if let Some(template) = semver_query.per_result_error_template.as_deref() {
            let message = config
                .handlebars()
                .render_template(template, &pretty_result)
                .context("Error instantiating semver query template.")
                .expect("could not materialize template");
            config.log_info(|config| {
                writeln!(config.stdout(), "  {}", message)?;
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
                    "\tlint rule output values:\n{}",
                    indented_serde
                )?;
                Ok(())
            })?;
        } else {
            config.log_info(|config| {
                writeln!(
                    config.stdout(),
                    "{}\n",
                    serde_json::to_string_pretty(&pretty_result)?
                )?;
                Ok(())
            })?;
        }

        if let Some(witness) = &semver_query.witness {
            if witness_generation.show_hints {
                let message = config
                    .handlebars()
                    .render_template(&witness.hint_template, &pretty_result)
                    .context("Error instantiating witness hint template.")?;

                config.log_info(|config| {
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
) -> anyhow::Result<CrateReport> {
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
        VersionChangeKind::Actual => format!("{}{} change", assume, change),
        VersionChangeKind::Minimum => format!("no change; {}{}", assume, change),
    };

    let index_storage = data_storage.create_indexes();
    let adapter = index_storage.create_adapter();

    let (queries_to_run, queries_to_skip): (Vec<_>, _) =
        SemverQuery::all_queries().into_values().partition(|query| {
            !version_change
                .level
                .supports_requirement(overrides.effective_required_update(query))
                && overrides.effective_lint_level(query) > LintLevel::Allow
        });
    let skipped_queries = queries_to_skip.len();

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
                    format_args!(
                        "{} checks, {} unnecessary",
                        queries_to_run.len(),
                        skipped_queries
                    ),
                )
            } else {
                config.shell_status(
                    "Starting",
                    format_args!(
                        "{} checks, {} unnecessary on {current_num_threads} threads",
                        queries_to_run.len(),
                        skipped_queries
                    ),
                )
            }
        })
        .expect("print failed");

    let queries_start_instant = Instant::now();
    let all_results = queries_to_run
        .par_iter()
        .map(|semver_query| {
            let start_instant = std::time::Instant::now();
            // trustfall::execute_query(...) -> dyn Iterator (without Send)
            // thus the result must be collect()'ed
            let results = adapter
                .run_query(&semver_query.query, semver_query.arguments.clone())?
                .collect_vec();
            let time_to_decide = start_instant.elapsed();
            Ok((semver_query, time_to_decide, results))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    if let Some(ref _witness_dir) = witness_generation.witness_directory {
        let _witness_results = all_results
            .par_iter()
            .map(
                |(semver_query, _, lint_results)| match semver_query.witness {
                    // Don't bother running the witness query unless both a witness query and template exist
                    Some(Witness {
                        witness_template: Some(_),
                        witness_query: Some(ref witness_query),
                        ..
                    }) => (
                        semver_query,
                        witness_gen::run_witness_queries(&adapter, witness_query, lint_results),
                    ),
                    // If no witness query exists, we still want to forward the existing output
                    _ => (
                        semver_query,
                        lint_results
                            .clone()
                            .into_iter()
                            .map(Result::Ok)
                            .collect_vec(),
                    ),
                },
            )
            .collect::<Vec<_>>();
    }

    let mut results_with_errors = vec![];
    let mut results_with_warnings = vec![];
    for (semver_query, time_to_decide, results) in all_results {
        config
            .log_verbose(|config| {
                let category = match overrides.effective_required_update(semver_query) {
                    RequiredSemverUpdate::Major => "major",
                    RequiredSemverUpdate::Minor => "minor",
                };

                let (status, status_color) = match (
                    results.is_empty(),
                    overrides.effective_lint_level(semver_query),
                ) {
                    (true, _) => ("PASS", AnsiColor::Green),
                    (false, LintLevel::Deny) => ("FAIL", AnsiColor::Red),
                    (false, LintLevel::Warn) => ("WARN", AnsiColor::Yellow),
                    (false, LintLevel::Allow) => unreachable!(
                        "`LintLevel::Allow` lint was unexpectedly not skipped: {semver_query:?}"
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
                    time_to_decide.as_secs_f32(),
                    category,
                    semver_query.id
                )?;
                Ok(())
            })
            .expect("print failed");

        if !results.is_empty() {
            match overrides.effective_lint_level(semver_query) {
                LintLevel::Deny => results_with_errors.push((semver_query, results)),
                LintLevel::Warn => results_with_warnings.push((semver_query, results)),
                LintLevel::Allow => unreachable!(
                    "`LintLevel::Allow` lint was unexpectedly not skipped: {semver_query:?}"
                ),
            };
        }
    }

    let produced_errors = !results_with_errors.is_empty();
    let produced_warnings = !results_with_warnings.is_empty();
    if produced_errors || produced_warnings {
        let status_color = if produced_errors {
            AnsiColor::Red
        } else {
            AnsiColor::Yellow
        };
        config
            .shell_print(
                "Checked",
                format_args!(
                    "[{:>8.3}s] {} checks: {} pass, {} fail, {} warn, {} skip",
                    queries_start_instant.elapsed().as_secs_f32(),
                    queries_to_run.len(),
                    queries_to_run.len() - results_with_errors.len() - results_with_warnings.len(),
                    results_with_errors.len(),
                    results_with_warnings.len(),
                    skipped_queries,
                ),
                Color::Ansi(status_color),
                true,
            )
            .expect("print failed");

        let mut required_versions = vec![];
        let mut suggested_versions = vec![];

        for (semver_query, results) in results_with_errors {
            required_versions.push(overrides.effective_required_update(semver_query));
            config.log_info(|config| {
                writeln!(
                    config.stdout(),
                    "\n--- failure {}: {} ---\n",
                    &semver_query.id,
                    &semver_query.human_readable_name
                )?;
                Ok(())
            })?;

            print_triggered_lint(config, semver_query, results, witness_generation)?;
        }

        for (semver_query, results) in results_with_warnings {
            suggested_versions.push(overrides.effective_required_update(semver_query));
            config.log_info(|config| {
                writeln!(
                    config.stdout(),
                    "\n--- warning {}: {} ---\n",
                    semver_query.id,
                    semver_query.human_readable_name
                )?;
                Ok(())
            })?;

            print_triggered_lint(config, semver_query, results, witness_generation)?;
        }

        let required_bump = required_versions.iter().max().copied();
        let suggested_bump = suggested_versions.iter().max().copied();

        if let Some(required_bump) = required_bump {
            writeln!(config.stderr())?;
            config.shell_print(
                "Summary",
                format_args!(
                    "semver requires new {} version: {} major and {} minor checks failed",
                    required_bump.as_str(),
                    required_versions
                        .iter()
                        .filter(|x| *x == &RequiredSemverUpdate::Major)
                        .count(),
                    required_versions
                        .iter()
                        .filter(|x| *x == &RequiredSemverUpdate::Minor)
                        .count(),
                ),
                Color::Ansi(AnsiColor::Red),
                true,
            )?;
        } else if produced_warnings {
            writeln!(config.stderr())?;
            config.shell_print(
                "Summary",
                "no semver update required",
                Color::Ansi(AnsiColor::Green),
                true,
            )?;
        } else {
            unreachable!("Expected either warnings or errors to be produced.");
        }

        if let Some(suggested_bump) = suggested_bump {
            config.shell_print(
                "Warning",
                format_args!(
                    "produced {} major and {} minor level warnings",
                    suggested_versions
                        .iter()
                        .filter(|x| *x == &RequiredSemverUpdate::Major)
                        .count(),
                    suggested_versions
                        .iter()
                        .filter(|x| *x == &RequiredSemverUpdate::Minor)
                        .count(),
                ),
                Color::Ansi(AnsiColor::Yellow),
                true,
            )?;

            if required_bump.is_none_or(|required_bump| required_bump < suggested_bump) {
                writeln!(
                    config.stderr(),
                    "{:12} produced warnings suggest new {} version",
                    "",
                    suggested_bump.as_str(),
                )?;
            }
        }

        Ok(CrateReport {
            required_bump: required_bump.map(ReleaseType::from),
            detected_bump: version_change.level,
        })
    } else {
        config
            .shell_print(
                "Checked",
                format_args!(
                    "[{:>8.3}s] {} checks: {} pass, {} skip",
                    queries_start_instant.elapsed().as_secs_f32(),
                    queries_to_run.len(),
                    queries_to_run.len(),
                    skipped_queries,
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

        Ok(CrateReport {
            detected_bump: version_change.level,
            required_bump: None,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
}
