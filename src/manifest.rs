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
pub(crate) struct LintTable {
    /// Optional key to indicate whether to opt-in to reading
    /// workspace lint configuration.  If not set in the TOML as
    /// `package.metadata.cargo-semver-checks.lints.workspace = true`,
    /// this field is set to `false. (note that setting the key in the
    /// TOML to false explicitly is invalid behavior and will be interpreted
    /// as just a missing field)
    ///
    /// Currently, we also read `lints.workspace`, but having this key
    /// in a Cargo.toml manifest is invalid if there is no `[workspace.lints]
    /// table in the workspace manifest.  Since we are storing our lint config in
    /// `[workspace.metadata.*]` for now, this could be the case.  If either this
    /// field is true or `lints.workspace` is set, we should read the workspace
    /// lint config.
    #[serde(default, deserialize_with = "deserialize_workspace_key")]
    pub(crate) workspace: bool,
    /// individual `lint_name = ...` entries
    #[serde(flatten, deserialize_with = "deserialize_into_overridemap")]
    pub(crate) inner: OverrideMap,
}

impl From<LintTable> for OverrideMap {
    fn from(value: LintTable) -> OverrideMap {
        value.inner
    }
}

/// Different valid representations of a [`QueryOverride`] in the Cargo.toml configuration table
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub(crate) enum OverrideConfig {
    /// Specify members by name, e.g.
    /// `lint_name = { level = "deny", required-update = "major" }
    /// Any omitted members will default to `None`
    #[serde(rename_all = "kebab-case")]
    Structure {
        #[serde(default)]
        level: Option<LintLevel>,
        #[serde(default)]
        required_update: Option<RequiredSemverUpdate>,
    },
    /// Shorthand for specifying just a lint level and leaving
    /// the other members as default: e.g.,
    /// `lint_name = "deny"`
    LintLevel(LintLevel),
}

impl From<OverrideConfig> for QueryOverride {
    fn from(value: OverrideConfig) -> Self {
        match value {
            OverrideConfig::Structure {
                level,
                required_update,
            } => Self {
                lint_level: level,
                required_update,
            },
            OverrideConfig::LintLevel(lint_level) => Self {
                lint_level: Some(lint_level),
                required_update: None,
            },
        }
    }
}

/// Lets serde deserialize a `BTreeMap<String, OverrideConfig>` into an [`OverrideMap`]
fn deserialize_into_overridemap<'de, D>(de: D) -> Result<OverrideMap, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    BTreeMap::<String, OverrideConfig>::deserialize(de)
        .map(|x| x.into_iter().map(|(k, v)| (k, v.into())).collect())
}

/// Deserializes the `workspace` key as an `Option<bool>`, raising
/// a hard error if `workspace = false` is explicity set, which is
/// an invalid configuration.  Returns a `bool` whether the workspace
/// key was explicitly set (`workspace = true`, return true) or omitted (false).
fn deserialize_workspace_key<'de, D>(de: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let option = Option::<bool>::deserialize(de)?;

    match option {
        Some(true) => Ok(true),
        None => Ok(false),
        Some(false) => Err(serde::de::Error::custom("`lints.workspace = false` is not valid configuration. Either set `lints.workspace = true` or omit the key entirely.")),
    }
}

/// Helper function to deserialize an optional lint table from a [`serde_json::Value`]
/// holding a `[package/workspace.metadata]` table holding a `cargo-semver-checks.lints` table
///
/// Returns an `Err` if the `cargo-semver-checks` table is present
/// but invalid.  Returns `Ok(None)` if the table is not present.
pub(crate) fn deserialize_lint_table(
    metadata: &serde_json::Value,
) -> anyhow::Result<Option<LintTable>> {
    let table = Option::<MetadataTable>::deserialize(metadata)?;
    Ok(table.and_then(|table| table.config.and_then(|config| config.lints)))
}

#[cfg(test)]
mod tests {

    use super::{LintTable, MetadataTable};
    use crate::QueryOverride;

    #[test]
    fn test_deserialize_config() {
        use crate::LintLevel::*;
        use crate::RequiredSemverUpdate::*;

        let manifest = r#"[package]
            name = "cargo-semver-checks"
            version = "1.2.3"
            edition = "2021"

            [package.metadata.cargo-semver-checks.lints]
            workspace = true
            two = "deny"
            three = { level = "warn" }
            four = { required-update = "major" }
            five = { required-update = "minor", level = "allow" }

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

        let pkg_table = package_metadata
            .config
            .expect("Semver checks table should be present")
            .lints
            .expect("Lint table should be present");

        assert!(
            pkg_table.workspace,
            "Package lints table should contain `workspace = true`"
        );
        let pkg = pkg_table.inner;

        let wks = workspace_metadata
            .config
            .expect("Semver checks table should be present")
            .lints
            .expect("Lint table should be present")
            .inner;

        assert!(
            matches!(
                pkg.get("two"),
                Some(&QueryOverride {
                    lint_level: Some(Deny),
                    required_update: None,
                })
            ),
            "got {:?}",
            pkg.get("two")
        );

        assert!(
            matches!(
                pkg.get("three"),
                Some(&QueryOverride {
                    required_update: None,
                    lint_level: Some(Warn)
                })
            ),
            "got {:?}",
            pkg.get("three")
        );

        assert!(
            matches!(
                pkg.get("four"),
                Some(&QueryOverride {
                    required_update: Some(Major),
                    lint_level: None,
                })
            ),
            "got {:?}",
            pkg.get("four")
        );

        //
        assert!(
            matches!(
                pkg.get("five"),
                Some(&QueryOverride {
                    required_update: Some(Minor),
                    lint_level: Some(Allow)
                })
            ),
            "got {:?}",
            pkg.get("five")
        );

        assert!(
            matches!(
                wks.get("six"),
                Some(&QueryOverride {
                    lint_level: Some(Allow),
                    required_update: None
                })
            ),
            "got {:?}",
            wks.get("six")
        );
    }

    #[test]
    fn workspace_key_false_is_error() {
        serde_json::from_value::<LintTable>(serde_json::json! {{
            "workspace": false
        }})
        .expect_err("`workspace = false` should not be accepted");
    }

    #[test]
    fn workspace_key_omitted_is_false() {
        let table = serde_json::from_value::<LintTable>(serde_json::json! {{
        }})
        .expect("this should be a valid lint table");
        assert!(!table.workspace, "table.workspace should be false");
    }
}
