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
    #[serde(flatten)]
    pub(crate) inner: BTreeMap<String, OverrideConfig>,
}

impl LintTable {
    /// Converts this into a stack of `OverrideMap`s, where entries at the top of the stack
    /// (later indices in the `Vec`) override entries lower in the stack.
    pub(crate) fn into_stack(self) -> Vec<OverrideMap> {
        // use a priority -> OverrideMap BTreeMap, which will be sorted by priority
        let mut map = BTreeMap::<_, OverrideMap>::new();
        for (id, config) in self.inner {
            let (priority, overrides) = match config {
                OverrideConfig::Shorthand(lint_level) => (
                    0,
                    QueryOverride {
                        lint_level: Some(lint_level),
                        required_update: None,
                    },
                ),
                OverrideConfig::Both {
                    level,
                    required_update,
                    priority,
                } => (
                    priority,
                    QueryOverride {
                        lint_level: Some(level),
                        required_update: Some(required_update),
                    },
                ),
                OverrideConfig::LintLevel { level, priority } => (
                    priority,
                    QueryOverride {
                        lint_level: Some(level),
                        required_update: None,
                    },
                ),
                OverrideConfig::RequiredUpdate {
                    required_update,
                    priority,
                } => (
                    priority,
                    QueryOverride {
                        lint_level: None,
                        required_update: Some(required_update),
                    },
                ),
            };

            map.entry(priority).or_default().insert(id, overrides);
        }

        // This will be sorted by key `priority` in ascending order.
        // To match the Cargo lint table semantics (more negative `priorities`
        // overrides more positive `priorities`) with the stack semantics
        // (later/greater-indexed elements at the top of the stack override
        // lower/lesser-indexed elements), we need to reverse this iterator,
        // so more negative `priority` keys come last at the top of the stack.
        map.into_values().rev().collect()
    }
}

/// Different valid representations of a [`QueryOverride`] in the Cargo.toml configuration table
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub(crate) enum OverrideConfig {
    /// Specify both lint level and required update by name, e.g.
    /// `lint_name = { level = "deny", required-update = "major" }
    #[serde(rename_all = "kebab-case")]
    Both {
        level: LintLevel,
        required_update: RequiredSemverUpdate,
        /// The priority for this configuration.  If there are multiple entries that
        /// configure a lint (e.g., a lint group containing a lint and the lint itself),
        /// the configuration entry with the **lowest** priority takes precedence.
        /// The default value, if omitted, is 0.
        #[serde(default)]
        priority: i64,
    },
    /// Specify just lint level by name, with optional priority.
    /// `lint_name = { level = "deny" }
    #[serde(rename_all = "kebab-case")]
    LintLevel {
        level: LintLevel,
        #[serde(default)]
        priority: i64,
    },
    /// Specify just required update by name, with optional priority.
    /// `lint_name = { required-update = "minor" }
    #[serde(rename_all = "kebab-case")]
    RequiredUpdate {
        required_update: RequiredSemverUpdate,
        #[serde(default)]
        priority: i64,
    },
    /// Shorthand for specifying just a lint level and leaving
    /// the other members (required_update and priority) as default: e.g.,
    /// `lint_name = "deny"`
    Shorthand(LintLevel),
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
        Some(false) => Err(serde::de::Error::custom(
            "`lints.workspace = false` is not valid configuration.\n\
            Either set `lints.workspace = true` or omit the key entirely.",
        )),
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
    use crate::{OverrideMap, QueryOverride};

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
            three = { level = "warn", priority = 1 }
            four = { required-update = "major", priority = 0 }
            five = { required-update = "minor", level = "allow", priority = -1 }

            [workspace.metadata.cargo-semver-checks.lints]
            six = "allow"
            seven = { level = "deny", priority = 2 }
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
        let pkg = pkg_table.into_stack();

        let wks = workspace_metadata
            .config
            .expect("Semver checks table should be present")
            .lints
            .expect("Lint table should be present")
            .into_stack();

        similar_asserts::assert_eq!(
            wks,
            vec![
                OverrideMap::from_iter([(
                    "seven".into(),
                    QueryOverride {
                        lint_level: Some(Deny),
                        required_update: None,
                    }
                ),]),
                OverrideMap::from_iter([(
                    "six".into(),
                    QueryOverride {
                        lint_level: Some(Allow),
                        required_update: None,
                    }
                ),]),
            ]
        );

        similar_asserts::assert_eq!(
            pkg,
            vec![
                OverrideMap::from_iter([(
                    "three".into(),
                    QueryOverride {
                        lint_level: Some(Warn),
                        required_update: None,
                    }
                )]),
                OverrideMap::from_iter([
                    (
                        "two".into(),
                        QueryOverride {
                            lint_level: Some(Deny),
                            required_update: None
                        }
                    ),
                    (
                        "four".into(),
                        QueryOverride {
                            lint_level: None,
                            required_update: Some(Major),
                        }
                    ),
                ]),
                OverrideMap::from_iter([(
                    "five".into(),
                    QueryOverride {
                        lint_level: Some(Allow),
                        required_update: Some(Minor),
                    }
                )])
            ]
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

    #[test]
    fn entry_with_no_fields_is_error() {
        toml::from_str::<LintTable>("one = {}").expect_err("one = {} should be invalid");

        toml::from_str::<LintTable>("one = { priority = 0 }")
            .expect_err("one = {priority = 0} should be invalid");
    }
}
