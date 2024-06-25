use std::collections::BTreeMap;

use anyhow::Context;
use serde::Deserialize;

use crate::{LintLevel, OverrideMap, QueryOverride, RequiredSemverUpdate};

#[derive(Debug, Clone)]
pub(crate) struct Manifest {
    pub(crate) path: std::path::PathBuf,
    pub(crate) parsed: cargo_toml::Manifest<MetadataTable>,
}

impl Manifest {
    pub(crate) fn parse(path: std::path::PathBuf) -> anyhow::Result<Self> {
        // Parsing via `cargo_toml::Manifest::from_path()` is preferable to parsing from a string,
        // because inspection of surrounding files is sometimes necessary to determine
        // the existence of lib targets and ensure proper handling of workspace inheritance.
        let parsed = cargo_toml::Manifest::from_path_with_metadata(&path)
            .with_context(|| format!("failed when reading {}", path.display()))?;

        Ok(Self { path, parsed })
    }
}

pub(crate) fn get_package_name(manifest: &Manifest) -> anyhow::Result<&str> {
    let package = manifest.parsed.package.as_ref().with_context(|| {
        format!(
            "failed to parse {}: no `package` table",
            manifest.path.display()
        )
    })?;
    Ok(&package.name)
}

pub(crate) fn get_package_version(manifest: &Manifest) -> anyhow::Result<&str> {
    let package = manifest.parsed.package.as_ref().with_context(|| {
        format!(
            "failed to parse {}: no `package` table",
            manifest.path.display()
        )
    })?;
    let version = package.version.get().with_context(|| {
        format!(
            "failed to retrieve package version from {}",
            manifest.path.display()
        )
    })?;
    Ok(version)
}

pub(crate) fn get_project_dir_from_manifest_path(
    manifest_path: &std::path::Path,
) -> anyhow::Result<std::path::PathBuf> {
    assert!(
        manifest_path.ends_with("Cargo.toml"),
        "path {} isn't pointing to a manifest",
        manifest_path.display()
    );
    let dir_path = manifest_path
        .parent()
        .context("manifest path doesn't have a parent")?;
    Ok(dir_path.to_path_buf())
}

/// A [package.metadata] or [workspace.metadata] table with
/// `cargo-semver-checks` config entries stored in the `config` field below.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct MetadataTable {
    /// Holds the `cargo-semver-checks` table, if it is declared.
    #[serde(default, rename = "cargo-semver-checks")]
    pub(crate) config: Option<SemverChecksTable>,
}

/// A `[cargo-semver-checks]` config table in `[package.metadata]`
/// or `[workspace.metadata]`.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub(crate) struct SemverChecksTable {
    /// Holds the `lints` table, if it is declared.
    pub(crate) lints: Option<LintTable>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct LintTable(BTreeMap<String, OverrideConfig>);

impl From<LintTable> for OverrideMap {
    fn from(value: LintTable) -> OverrideMap {
        value.0.into_iter().map(|(k, v)| (k, v.into())).collect()
    }
}

/// Different valid representations of a [`QueryOverride`] in the Cargo.toml configuration table
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged, rename_all = "kebab-case")]
pub(crate) enum OverrideConfig {
    /// Specify members by name, e.g.
    /// `lint_name = { lint-level = "deny", required-update = "major" }
    /// Any omitted members will default to `None`
    Structure(QueryOverride),
    /// Shorthand for specifying just a lint level and leaving
    /// the other members as default: e.g.,
    /// `lint_name = "deny"`
    LintLevel(LintLevel),
    /// Shorthand for specifying just a required version bump and leaving
    /// the other members as default: e.g.,
    /// `lint_name = "minor"`
    RequiredUpdate(RequiredSemverUpdate),
}

impl From<OverrideConfig> for QueryOverride {
    fn from(value: OverrideConfig) -> Self {
        match value {
            OverrideConfig::Structure(x) => x,
            OverrideConfig::LintLevel(lint_level) => Self {
                lint_level: Some(lint_level),
                required_update: None,
            },
            OverrideConfig::RequiredUpdate(required_update) => Self {
                lint_level: None,
                required_update: Some(required_update),
            },
        }
    }
}

/// Helper function to deserialize an optional lint table from a [`serde_json::Value`]
/// into a [`OverrideMap`].  Returns an `Err` if the `cargo-semver-checks` table is present
/// but invalid.  Returns `Ok(None)` if the table is not present.
pub(crate) fn deserialize_lint_table(
    metadata: &serde_json::Value,
) -> anyhow::Result<Option<OverrideMap>> {
    let table = Option::<MetadataTable>::deserialize(metadata)?;
    Ok(table.and_then(|table| {
        table
            .config
            .and_then(|config| config.lints.map(OverrideMap::from))
    }))
}

#[cfg(test)]
mod tests {
    use crate::{manifest::OverrideConfig, QueryOverride};

    use super::MetadataTable;

    #[test]
    fn test_deserialize_config() {
        use crate::LintLevel::*;
        use crate::RequiredSemverUpdate::*;
        use OverrideConfig::*;
        let manifest = r#"[package]
            name = "cargo-semver-checks"
            version = "1.2.3"
            edition = "2021"

            [package.metadata.cargo-semver-checks.lints]
            one = "major"
            two = "deny"
            three = { lint-level = "warn" }
            four = { required-update = "major" }
            five = { required-update = "minor", lint-level = "allow" }

            [workspace.metadata.cargo-semver-checks.lints]
            six = "allow"
            "#;

        let parsed = cargo_toml::Manifest::from_slice_with_metadata(manifest.as_bytes())
            .expect("Cargo.toml should be valid");
        let package_metadata: MetadataTable = parsed
            .package
            .expect("Cargo.toml should contain a package")
            .metadata
            .expect("Package metadata should be present");

        let workspace_metadata = parsed
            .workspace
            .expect("Cargo.toml should contain a workspace")
            .metadata
            .expect("Workspace metadata should be present");

        let pkg = package_metadata
            .config
            .expect("Semver checks table should be present")
            .lints
            .expect("Lint table should be present")
            .0;
        let wks = workspace_metadata
            .config
            .expect("Semver checks table should be present")
            .lints
            .expect("Lint table should be present")
            .0;
        assert!(
            matches!(pkg.get("one"), Some(&RequiredUpdate(Major))),
            "got {:?}",
            pkg.get("one")
        );

        assert!(
            matches!(pkg.get("two"), Some(&LintLevel(Deny))),
            "got {:?}",
            pkg.get("two")
        );

        assert!(
            matches!(
                pkg.get("three"),
                Some(&Structure(QueryOverride {
                    required_update: None,
                    lint_level: Some(Warn)
                }))
            ),
            "got {:?}",
            pkg.get("three")
        );

        assert!(
            matches!(
                pkg.get("four"),
                Some(&Structure(QueryOverride {
                    required_update: Some(Major),
                    lint_level: None,
                }))
            ),
            "got {:?}",
            pkg.get("four")
        );

        //
        assert!(
            matches!(
                pkg.get("five"),
                Some(&Structure(QueryOverride {
                    required_update: Some(Minor),
                    lint_level: Some(Allow)
                }))
            ),
            "got {:?}",
            pkg.get("five")
        );

        assert!(
            matches!(wks.get("six"), Some(&LintLevel(Allow))),
            "got {:?}",
            wks.get("six")
        );
    }
}
