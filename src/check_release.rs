use std::{collections::BTreeMap, env, io::Write, iter::Peekable, sync::Arc, time::Duration};

use anyhow::Context;
use clap::crate_version;
use itertools::Itertools;
use termcolor::Color;
use termcolor_output::{colored, colored_ln};
use trustfall::{FieldValue, TransparentValue};
use trustfall_rustdoc::{VersionedCrate, VersionedIndexedCrate, VersionedRustdocAdapter};

use crate::{
    query::{ActualSemverUpdate, RequiredSemverUpdate, SemverQuery},
    CrateReport, GlobalConfig, ReleaseType,
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

pub(super) fn run_check_release(
    config: &mut GlobalConfig,
    crate_name: &str,
    current_crate: VersionedCrate,
    baseline_crate: VersionedCrate,
    release_type: Option<ReleaseType>,
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

    let queries = SemverQuery::all_queries();

    let current = VersionedIndexedCrate::new(&current_crate);
    let previous = VersionedIndexedCrate::new(&baseline_crate);
    let adapter = VersionedRustdocAdapter::new(&current, Some(&previous))?;
    let mut queries_with_errors: Vec<QueryWithResults> = vec![];

    let queries_to_run: Vec<_> = queries
        .iter()
        .filter(|(_, query)| !version_change.supports_requirement(query.required_update))
        .collect();
    let skipped_queries = queries.len().saturating_sub(queries_to_run.len());

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
        .verbose(|config| {
            config.shell_status(
                "Starting",
                format_args!(
                    "{} checks, {} unnecessary",
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
        let mut results_iter = adapter
            .run_query(&semver_query.query, semver_query.arguments.clone())?
            .peekable();
        let peeked = results_iter.peek();
        let time_to_decide = start_instant.elapsed();
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
                    "[{:>8.3}s] {} checks; {} passed, {} failed, {} unnecessary",
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
            config
                .log(|config| {
                    colored_ln(config.stdout(), |w| {
                        colored!(
                            w,
                            "\n--- failure {}: {} ---\n",
                            &semver_query.id,
                            &semver_query.human_readable_name,
                        )
                    })?;
                    Ok(())
                })
                .expect("print failed");

            if let Some(ref_link) = semver_query.reference_link.as_deref() {
                config
                .log(|config| {
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
                            "https://github.com/obi1kenobi/cargo-semver-checks/tree/v{}/src/lints/{}.ron",
                            crate_version!(),
                            semver_query.id,
                        )
                    )
                })?;
                Ok(())
            })
                .expect("print failed");
            } else {
                config
                .log(|config| {
                    colored_ln(config.stdout(), |w| {
                        colored!(
                            w,
                            "{}Description:{}\n{}\n{:>12} {}\n",
                            bold!(true),
                            reset!(),
                            &semver_query.error_message,
                            "impl:",
                            format!(
                                "https://github.com/obi1kenobi/cargo-semver-checks/tree/v{}/src/lints/{}.ron",
                                crate_version!(),
                                semver_query.id,
                            )
                        )
                    })?;
                    Ok(())
                })
                .expect("print failed");
            }

            config
                .log(|config| {
                    colored_ln(config.stdout(), |w| {
                        colored!(w, "{}Failed in:{}", bold!(true), reset!())
                    })?;
                    Ok(())
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
                        .context("Error instantiating semver query template.")
                        .expect("could not materialize template");
                    config
                        .log(|config| {
                            colored_ln(config.stdout(), |w| colored!(w, "  {}", message,))?;
                            Ok(())
                        })
                        .expect("print failed");

                    config
                        .extra_verbose(|config| {
                            colored_ln(config.stdout(), |w| {
                                let serde_pretty = serde_json::to_string_pretty(&pretty_result)
                                    .expect("serde failed");
                                let indented_serde = serde_pretty
                                    .split('\n')
                                    .map(|line| format!("    {line}"))
                                    .join("\n");
                                colored!(w, "    lint rule output values:\n{}", indented_serde)
                            })
                            .map_err(|e| e.into())
                        })
                        .expect("print failed");
                } else {
                    config
                        .log(|config| {
                            colored_ln(config.stdout(), |w| {
                                colored!(
                                    w,
                                    "{}\n",
                                    serde_json::to_string_pretty(&pretty_result)
                                        .expect("serde failed"),
                                )
                            })
                            .expect("print failed");
                            Ok(())
                        })
                        .expect("print failed");
                }
            }
            total_duration += start_instant.elapsed();
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
                "Final",
                format_args!(
                    "[{:>8.3}s] semver requires new {} version: {} major and {} minor checks failed",
                    total_duration.as_secs_f32(),
                    required_bump.as_str(),
                    required_versions.iter().filter(|x| *x == &RequiredSemverUpdate::Major).count(),
                    required_versions.iter().filter(|x| *x == &RequiredSemverUpdate::Minor).count(),
                ),
                Color::Red,
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
                "Completed",
                format_args!(
                    "[{:>8.3}s] {} checks; {} passed, {} unnecessary",
                    total_duration.as_secs_f32(),
                    queries_to_run.len(),
                    queries_to_run.len(),
                    skipped_queries,
                ),
                Color::Green,
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
