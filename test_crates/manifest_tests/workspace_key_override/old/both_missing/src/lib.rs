#![no_std]

// This is removed in the new version to trigger the `struct_missing` lint
// This is set to allow in `[workspace.metadata.cargo-semver-checks.lints]`,
// and since this *package* is missing the `lints.workspace` key, _and_ the
// `package.metadata.cargo-semver-cheks.lints.workspace = true`, this override
// should not be applied.
pub struct StructMissing;
