use std::io::Write as _;
use std::{collections::BTreeMap, sync::Arc, time::Instant};

use anstyle::{AnsiColor, Color, Reset, Style};

use anyhow::Context;
use clap::crate_version;
use itertools::Itertools;
use rayon::prelude::*;
use trustfall::{FieldValue, TransparentValue};
use trustfall_rustdoc::{VersionedCrate, VersionedIndexedCrate, VersionedRustdocAdapter};

use crate::{
    query::{ActualSemverUpdate, OverrideStack, RequiredSemverUpdate, SemverQuery},
    CrateReport, GlobalConfig, ReleaseType,
};

fn classify_semver_version_change(
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
        } else if baseline_version.pre != current_version.pre {
            // > A pre-release version indicates that the version is unstable and might not satisfy
            // > the intended compatibility requirements as denoted by its associated normal version
            // https://semver.org/#spec-item-9
            ActualSemverUpdate::Major
        } else {
            ActualSemverUpdate::NotChanged
        };

        Some(update_kind)
    } else {
        None
    }
}

/// Helper function to print an error message about a failed lint.
fn print_failed_lint(
    config: &mut GlobalConfig,
    semver_query: &SemverQuery,
    results: Vec<BTreeMap<Arc<str>, FieldValue>>,
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
    }

    Ok(())
}

pub(super) fn run_check_release(
    config: &mut GlobalConfig,
    crate_name: &str,
    current_crate: VersionedCrate,
    baseline_crate: VersionedCrate,
    release_type: Option<ReleaseType>,
    overrides: &OverrideStack,
) -> anyhow::Result<CrateReport> {
    let current_version = current_crate.crate_version();
    let baseline_version = baseline_crate.crate_version();

    let version_change = release_type
        .map(Into::into)
        .or_else(|| classify_semver_version_change(current_version, baseline_version))
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
    let assume = match release_type {
        Some(_) => "assume ",
        None => "",
    };

    let current = VersionedIndexedCrate::new(&current_crate);
    let previous = VersionedIndexedCrate::new(&baseline_crate);
    let adapter = VersionedRustdocAdapter::new(&current, Some(&previous))?;

    let (queries_to_run, queries_to_skip): (Vec<_>, _) =
        SemverQuery::all_queries().into_values().partition(|query| {
            !version_change.supports_requirement(overrides.effective_required_update(query))
        });
    let skipped_queries = queries_to_skip.len();

    config.shell_status(
        "Checking",
        format_args!(
            "{crate_name} v{} -> v{} ({}{} change)",
            baseline_version.unwrap_or("unknown"),
            current_version.unwrap_or("unknown"),
            assume,
            change
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

    let mut results_with_errors = vec![];
    for (semver_query, time_to_decide, results) in all_results {
        config
            .log_verbose(|config| {
                let category = match overrides.effective_required_update(semver_query) {
                    RequiredSemverUpdate::Major => "major",
                    RequiredSemverUpdate::Minor => "minor",
                };
                if results.is_empty() {
                    writeln!(
                        config.stderr(),
                        "{}{:>12}{} [{:8.3}s] {:^18} {}",
                        Style::new()
                            .fg_color(Some(Color::Ansi(AnsiColor::Green)))
                            .bold(),
                        "PASS",
                        Reset,
                        time_to_decide.as_secs_f32(),
                        category,
                        semver_query.id
                    )?;
                } else {
                    writeln!(
                        config.stderr(),
                        "{}{:>12}{} [{:>8.3}s] {:^18} {}",
                        Style::new()
                            .fg_color(Some(Color::Ansi(AnsiColor::Red)))
                            .bold(),
                        "FAIL",
                        Reset,
                        time_to_decide.as_secs_f32(),
                        category,
                        semver_query.id
                    )?;
                }
                Ok(())
            })
            .expect("print failed");
        if !results.is_empty() {
            results_with_errors.push((semver_query, results));
        }
    }

    if !results_with_errors.is_empty() {
        config
            .shell_print(
                "Checked",
                format_args!(
                    "[{:>8.3}s] {} checks; {} passed, {} failed, {} unnecessary",
                    queries_start_instant.elapsed().as_secs_f32(),
                    queries_to_run.len(),
                    queries_to_run.len() - results_with_errors.len(),
                    results_with_errors.len(),
                    skipped_queries,
                ),
                Color::Ansi(AnsiColor::Red),
                true,
            )
            .expect("print failed");

        let mut required_versions = vec![];

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

            print_failed_lint(config, semver_query, results)?;
        }

        let required_bump = if required_versions.contains(&RequiredSemverUpdate::Major) {
            RequiredSemverUpdate::Major
        } else if required_versions.contains(&RequiredSemverUpdate::Minor) {
            RequiredSemverUpdate::Minor
        } else {
            unreachable!("{:?}", required_versions)
        };

        config
            .shell_print(
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
            )
            .expect("print failed");

        Ok(CrateReport {
            required_bump: Some(required_bump.into()),
            detected_bump: version_change,
        })
    } else {
        config
            .shell_print(
                "Checked",
                format_args!(
                    "[{:>8.3}s] {} checks; {} passed, {} unnecessary",
                    queries_start_instant.elapsed().as_secs_f32(),
                    queries_to_run.len(),
                    queries_to_run.len(),
                    skipped_queries,
                ),
                Color::Ansi(AnsiColor::Green),
                true,
            )
            .expect("print failed");
        Ok(CrateReport {
            detected_bump: version_change,
            required_bump: None,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn classify_no_version() {
        let baseline = None;
        let current = None;
        let expected = None;
        let actual = classify_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_same_version() {
        let baseline = Some("1.0.0");
        let current = Some("1.0.0");
        let expected = Some(ActualSemverUpdate::NotChanged);
        let actual = classify_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_patch_changed() {
        let baseline = Some("1.0.0");
        let current = Some("1.0.1");
        let expected = Some(ActualSemverUpdate::Patch);
        let actual = classify_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_minor_changed() {
        let baseline = Some("1.0.0");
        let current = Some("1.1.0");
        let expected = Some(ActualSemverUpdate::Minor);
        let actual = classify_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_major_changed() {
        let baseline = Some("0.9.0");
        let current = Some("1.0.0");
        let expected = Some(ActualSemverUpdate::Major);
        let actual = classify_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_zerover_minor_changed() {
        let baseline = Some("0.1.0");
        let current = Some("0.1.1");
        let expected = Some(ActualSemverUpdate::Minor);
        let actual = classify_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_zerover_major_changed() {
        let baseline = Some("0.1.0");
        let current = Some("0.2.0");
        let expected = Some(ActualSemverUpdate::Major);
        let actual = classify_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_double_zerover_major_changed() {
        let baseline = Some("0.0.1");
        let current = Some("0.0.2");
        let expected = Some(ActualSemverUpdate::Major);
        let actual = classify_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_pre_same() {
        let baseline = Some("1.0.0-alpha.0");
        let current = Some("1.0.0-alpha.0");
        let expected = Some(ActualSemverUpdate::NotChanged);
        let actual = classify_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_pre() {
        let baseline = Some("1.0.0-alpha.0");
        let current = Some("1.0.0-alpha.1");
        let expected = Some(ActualSemverUpdate::Major);
        let actual = classify_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_same_version_with_pre() {
        let baseline = Some("1.0.0-alpha.1");
        let current = Some("1.0.0");
        let expected = Some(ActualSemverUpdate::Major);
        let actual = classify_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_minor_changed_with_pre() {
        let baseline = Some("1.0.0");
        let current = Some("1.1.0-alpha.1");
        let expected = Some(ActualSemverUpdate::Minor);
        let actual = classify_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }

    #[test]
    fn classify_ignores_build() {
        let baseline = Some("1.0.0+hello");
        let current = Some("1.0.0+world");
        let expected = Some(ActualSemverUpdate::NotChanged);
        let actual = classify_semver_version_change(baseline, current);
        assert_eq!(actual, expected);
    }
}
