//! Helpers for rendering cargo-script witnesses.
//!
//! A rendered witness is a self-contained cargo-script source file with:
//! - an embedded TOML manifest frontmatter,
//! - two dependency profiles under `[dependencies]` (`baseline` and `current`),
//! - the witness body at module scope, and
//! - an intentionally empty `main()`.
//!
//! Witness validation uses `cargo check`, not runtime execution, so the body is
//! structured to demonstrate type-checking breakage while `main()` remains
//! empty and side-effect free.

use std::{collections::BTreeMap, sync::Arc};

use anyhow::{Context, Result};
use fs_err as fs;
use handlebars::Handlebars;
use trustfall::FieldValue;
use trustfall_rustdoc::VersionedRustdocAdapter;

use crate::{data_generation::CrateDataRequest, query::Witness};

use super::{SCRIPT_SHEBANG, WitnessGenerationData, generate_witness_text, run_witness_query};

/// Fully rendered witness sources for one lint result.
///
/// This type keeps both the human-facing retained script and the exact
/// baseline/current variants that `cargo check` validates. The baseline/current
/// variants differ only in which dependency profile is commented out.
#[derive(Debug)]
pub(super) struct GeneratedWitnessScript {
    /// Canonical retained witness source shown to users and saved in artifacts.
    ///
    /// This cargo script enables the baseline dependency profile and comments
    /// out the current profile so the file is directly readable as "baseline
    /// works, current is the alternate configuration".
    pub(super) witness_rs: String,
    /// Exact script compiled against the baseline crate.
    ///
    /// Today this is identical to [`Self::witness_rs`], but it is stored
    /// separately so the retained bundle always contains the verbatim executed
    /// input rather than relying on callers to reconstruct it.
    pub(super) baseline_rs: String,
    /// Exact script compiled against the current crate.
    ///
    /// Structurally identical to [`Self::baseline_rs`] except that the
    /// dependency profile toggle is reversed.
    pub(super) current_rs: String,
    /// Rust crate-root name used by the witness body and dependency key.
    ///
    /// This is intentionally the downstream Rust import name, which may differ
    /// from the Cargo package name when a crate uses a renamed `lib` target or
    /// when the package name contains dashes.
    pub(super) crate_root_name: String,
    /// Dependency profile that makes the baseline crate active.
    ///
    /// Stored separately so artifacts and manifests can describe the executed
    /// dependency configuration without having to reparse script text.
    pub(super) baseline_dependency: DependencyProfile,
    /// Dependency profile that makes the current crate active.
    pub(super) current_dependency: DependencyProfile,
}

/// Dependency lines for one witness profile.
///
/// The lines are stored as a vector even though current witnesses usually emit
/// one dependency entry per profile. That preserves the design choice that a
/// "profile" may later expand to a group of coordinated dependency entries
/// without changing the surrounding APIs.
#[derive(Debug, Clone)]
pub(super) struct DependencyProfile {
    pub(super) lines: Vec<String>,
}

/// Renders the cargo-script witness sources for one lint result.
///
/// The returned scripts share the same structure: TOML frontmatter with two
/// dependency profiles, witness code at module scope, and an empty `main()`.
/// Callers should use this when they need the exact sources that witness
/// execution and retained artifacts will consume.
pub(super) fn build_witness_script(
    handlebars: &Handlebars,
    witness_data: &WitnessGenerationData<'_>,
    adapter: &VersionedRustdocAdapter,
    witness: &Witness,
    query_result: &BTreeMap<Arc<str>, FieldValue>,
) -> Result<GeneratedWitnessScript> {
    let baseline = witness_data
        .baseline
        .context("cannot generate witness: missing baseline crate data request")?;
    let current = witness_data
        .current
        .context("cannot generate witness: missing current crate data request")?;

    let witness_template = witness
        .witness_template
        .as_deref()
        .context("cannot generate witness: missing witness template")?;

    let witness_results = match witness.witness_query.as_ref() {
        Some(witness_query) => run_witness_query(adapter, witness_query, query_result.clone())
            .context("error running witness query")?,
        None => query_result.clone(),
    };
    let crate_root_name = determine_crate_root_name(&witness_results, baseline)?;
    let witness_body = generate_witness_text(handlebars, witness_template, witness_results)
        .with_context(|| "error rendering witness template")?;

    let baseline_dependency = dependency_profile(baseline, &crate_root_name)?;
    let current_dependency = dependency_profile(current, &crate_root_name)?;

    Ok(render_script_variants(
        &crate_root_name,
        &baseline_dependency,
        &current_dependency,
        &witness_body,
    ))
}

/// Builds the dependency stanza for one side of a witness script.
///
/// Callers should pass the crate's real downstream Rust import name as
/// `dependency_name` instead of a synthetic alias. This keeps retained witness
/// scripts close to what a user would write by hand, while `package = ...`
/// still points Cargo at the original package when the package name and import
/// name differ.
pub(super) fn dependency_profile(
    request: &CrateDataRequest<'_>,
    dependency_name: &str,
) -> Result<DependencyProfile> {
    let package_name = request.package_name()?.to_owned();
    let mut fields = vec![format!("package = {}", toml_string(&package_name))];
    // Git revisions have already been extracted into a local tree by this
    // point, so they flow through the local path dependency case below.
    if let Some(project_dir) = request.local_project_dir()? {
        let canonical_dir = fs::canonicalize(&project_dir).with_context(|| {
            format!(
                "failed to canonicalize witness path dependency {}",
                project_dir.display()
            )
        })?;
        fields.push(format!(
            "path = {}",
            toml_string(&canonical_dir.to_string_lossy())
        ));
    } else {
        fields.push(format!(
            "version = {}",
            toml_string(&request.exact_version()?)
        ));
    }

    if !request.default_features_enabled() {
        fields.push("default-features = false".to_owned());
    }

    let extra_features = request
        .extra_features()
        .map(toml_string)
        .collect::<Vec<_>>();
    if !extra_features.is_empty() {
        fields.push(format!("features = [{}]", extra_features.join(", ")));
    }

    let line = format!("{dependency_name} = {{ {} }}", fields.join(", "));

    Ok(DependencyProfile { lines: vec![line] })
}

pub(super) fn determine_crate_root_name(
    query_result: &BTreeMap<Arc<str>, FieldValue>,
    baseline: &CrateDataRequest<'_>,
) -> Result<String> {
    if let Some(root_name) = extract_path_root(query_result)
        && is_supported_identifier(&root_name)
    {
        return Ok(root_name);
    }

    let fallback = baseline.fallback_import_name()?;
    if is_supported_identifier(&fallback) {
        Ok(fallback)
    } else {
        anyhow::bail!("could not determine a valid Rust crate root name for witness generation");
    }
}

fn extract_path_root(query_result: &BTreeMap<Arc<str>, FieldValue>) -> Option<String> {
    let value = query_result.get("path")?;

    if let Some(segments) = value.as_vec_with(FieldValue::as_str) {
        return segments.first().map(|segment| (*segment).to_owned());
    }

    value
        .as_vec_with(|entry| entry.as_vec_with(FieldValue::as_str))
        .and_then(|paths| paths.first().and_then(|path| path.first().copied()))
        .map(str::to_owned)
}

fn is_supported_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };

    if !(first == '_' || first.is_ascii_alphabetic()) {
        return false;
    }

    chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

/// Renders the canonical retained witness script plus the exact executed
/// baseline/current variants derived from it.
///
/// Callers should treat `witness.rs` as the human-facing source of truth and
/// `baseline.rs` / `current.rs` as the compiled variants. The executed variants
/// differ only in which dependency profile is commented out.
pub(super) fn render_script_variants(
    crate_root_name: &str,
    baseline_dependency: &DependencyProfile,
    current_dependency: &DependencyProfile,
    witness_body: &str,
) -> GeneratedWitnessScript {
    let witness_rs = render_script(
        &baseline_dependency.lines,
        &current_dependency.lines,
        witness_body,
        DependencySet::Baseline,
    );
    let baseline_rs = witness_rs.clone();
    let current_rs = render_script(
        &baseline_dependency.lines,
        &current_dependency.lines,
        witness_body,
        DependencySet::Current,
    );

    GeneratedWitnessScript {
        witness_rs,
        baseline_rs,
        current_rs,
        crate_root_name: crate_root_name.to_owned(),
        baseline_dependency: baseline_dependency.clone(),
        current_dependency: current_dependency.clone(),
    }
}

#[derive(Debug, Clone, Copy)]
enum DependencySet {
    Baseline,
    Current,
}

fn render_script(
    baseline_dependency_lines: &[String],
    current_dependency_lines: &[String],
    witness_body: &str,
    dependency_set: DependencySet,
) -> String {
    let mut lines = vec![
        SCRIPT_SHEBANG.to_owned(),
        "---".to_owned(),
        r#"package.edition = "2024""#.to_owned(),
        String::new(),
        "[dependencies]".to_owned(),
    ];
    lines.extend(render_dependency_block(
        "baseline",
        baseline_dependency_lines,
        matches!(dependency_set, DependencySet::Baseline),
    ));
    lines.push(String::new());
    lines.extend(render_dependency_block(
        "current",
        current_dependency_lines,
        matches!(dependency_set, DependencySet::Current),
    ));
    lines.push("---".to_owned());
    lines.push(String::new());
    lines.push("#![allow(warnings)]".to_owned());
    lines.push(String::new());

    // The witness body lives at module scope so compilation alone is enough to
    // validate it. `main()` stays empty on purpose: witnesses demonstrate
    // type-checking breakage, not runtime behavior.
    lines.push(witness_body.trim().to_owned());
    lines.push(String::new());
    lines.push("fn main() {}".to_owned());
    lines.push(String::new());

    lines.join("\n")
}

fn render_dependency_block(label: &str, dependency_lines: &[String], enabled: bool) -> Vec<String> {
    let mut block = vec![format!("# cargo-semver-checks profile: {label}")];
    block.extend(dependency_lines.iter().map(|line| {
        if enabled {
            line.clone()
        } else {
            format!("# {line}")
        }
    }));
    block
}

fn toml_string(value: &str) -> String {
    toml::Value::String(value.to_owned()).to_string()
}
