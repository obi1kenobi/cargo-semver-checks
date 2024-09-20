use std::io::Write as _;
use std::{collections::BTreeMap, sync::Arc, time::Instant};

use annotate_snippets::{Level, Renderer, Snippet};
use anstyle::{AnsiColor, Color, Reset, Style};

use anyhow::Context;
use clap::crate_version;
use itertools::Itertools;
use rayon::prelude::*;
use trustfall::{FieldValue, TransparentValue};
use trustfall_rustdoc::{VersionedCrate, VersionedIndexedCrate, VersionedRustdocAdapter};

use crate::{
    query::{ActualSemverUpdate, LintLevel, OverrideStack, RequiredSemverUpdate, SemverQuery},
    CrateReport, GlobalConfig, ReleaseType, WitnessGeneration,
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

/// Helper function to print details about a triggered lint.
fn print_triggered_lint(
    config: &mut GlobalConfig,
    semver_query: &SemverQuery,
    results: Vec<BTreeMap<Arc<str>, FieldValue>>,
    witness_generation: &WitnessGeneration,
    level: LintLevel,
) -> anyhow::Result<()> {
    let snippets_level = match level {
        LintLevel::Deny => Level::Error,
        LintLevel::Warn => Level::Warning,
        LintLevel::Allow => unreachable!("Called `print_triggered_lint` on an `allow`-level lint"),
    };

    let title = format!("{}: {}", semver_query.id, semver_query.human_readable_name);
    let mut message = snippets_level.title(&title);
    let mut description = Level::Info.title(&semver_query.error_message);

    let ref_link_fmt;
    if let Some(ref_link) = semver_query.reference_link.as_deref() {
        ref_link_fmt = format!("ref: {ref_link}");
        description = description.footer(Level::Help.title(&ref_link_fmt));
    }

    let impl_link = format!(
        "impl: https://github.com/obi1kenobi/cargo-semver-checks/tree/v{}/src/lints/{}.ron",
        crate_version!(),
        semver_query.id
    );
    description = description.footer(Level::Help.title(&impl_link));

    message = message.footer(description);

    #[derive(Debug, Default)]
    struct ResultFormat {
        error_message: Option<String>,
        /// (text source, 1-indexed line number to error, filename)
        span: Option<(String, usize, String)>,
        serde_pretty: Option<String>,
        witness_hint: Option<String>,
    }

    let formats: Vec<_> = results
        .into_iter()
        .map(|semver_violation_result| {
            let mut result = ResultFormat::default();

            if let Some(filename) = semver_violation_result
                .get("span_filename")
                .and_then(FieldValue::as_str)
            {
                if let Some(begin_line) = semver_violation_result
                    .get("span_begin_line")
                    .and_then(FieldValue::as_usize)
                {
                    match std::fs::read_to_string(filename) {
                        Ok(contents) => {
                            result.span = Some((contents, begin_line, filename.to_owned()));
                        }
                        Err(e) => config
                            .log_verbose(|config| {
                                config.shell_warn(format_args!(
                                    "could not read file `{filename}`: {e}"
                                ))
                            })
                            .expect("printing failed"),
                    }
                }
            }

            let pretty_result: BTreeMap<Arc<str>, TransparentValue> = semver_violation_result
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect();

            if let Some(template) = semver_query.per_result_error_template.as_deref() {
                let error_message = config
                    .handlebars()
                    .render_template(template, &pretty_result)
                    .context("Error instantiating semver query template.")
                    .expect("could not materialize template");

                result.error_message = Some(format!("failed in: {error_message}"));

                if config.is_extra_verbose() {
                    result.serde_pretty = Some(
                        serde_json::to_string_pretty(&pretty_result)
                            .expect("error serializing pretty results"),
                    );
                }
            } else {
                result.serde_pretty = Some(
                    serde_json::to_string_pretty(&pretty_result)
                        .expect("error serializing pretty results"),
                );
            }

            if let Some(witness) = &semver_query.witness {
                if witness_generation.show_hints {
                    result.witness_hint = Some(
                        config
                            .handlebars()
                            .render_template(&witness.hint_template, &pretty_result)
                            .expect("Error instantiating witness hint template."),
                    );
                }
            }
            result
        })
        .collect();

    for format_result in &formats {
        let mut failure = if let Some(error_message) = &format_result.error_message {
            snippets_level.title(error_message)
        } else {
            snippets_level.title("failed in:")
        };

        if let Some((source, begin_line, filename)) = &format_result.span {
            // annotate_snippets wants the byte index to highlight
            let (_, _, start, end) = source.as_bytes().into_iter().fold(
                (1, 0, 0, 0),
                |(mut line, mut len, mut start, mut end), c| {
                    len += 1;
                    if let b'\n' | b'\r' = c {
                        if line == *begin_line {
                            end = len;
                        }
                        line += 1;
                        if line == *begin_line {
                            start = len;
                        }
                    }

                    (line, len, start, end)
                },
            );

            failure = failure.snippet(
                Snippet::source(source)
                    .origin(filename)
                    .fold(true)
                    .annotation(snippets_level.span(start..end)),
            );
        }

        if let Some(serde_pretty) = &format_result.serde_pretty {
            failure = failure.footer(
                Level::Info
                    .title("lint rule output values")
                    .snippet(Snippet::source(serde_pretty)),
            );
        }

        if let Some(witness_hint) = &format_result.witness_hint {
            failure = failure.footer(
                Level::Note
                    .title("downstream code similar to the following would break")
                    .snippet(Snippet::source(&witness_hint)),
            );
        }

        message = message.footer(failure);
    }

    let renderer = Renderer::styled();
    config.log_info(|config| {
        writeln!(config.stdout(), "{}\n", renderer.render(message))?;
        Ok(())
    })
}

pub(super) fn run_check_release(
    config: &mut GlobalConfig,
    crate_name: &str,
    current_crate: VersionedCrate,
    baseline_crate: VersionedCrate,
    release_type: Option<ReleaseType>,
    overrides: &OverrideStack,
    witness_generation: &WitnessGeneration,
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
                && overrides.effective_lint_level(query) > LintLevel::Allow
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

            print_triggered_lint(
                config,
                semver_query,
                results,
                witness_generation,
                LintLevel::Deny,
            )?;
        }

        for (semver_query, results) in results_with_warnings {
            suggested_versions.push(overrides.effective_required_update(semver_query));
            print_triggered_lint(
                config,
                semver_query,
                results,
                witness_generation,
                LintLevel::Warn,
            )?;
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

            if required_bump.map_or(true, |required_bump| required_bump < suggested_bump) {
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
            detected_bump: version_change,
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
