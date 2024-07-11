// This is removed in the new version to trigger the `struct_missing` lint
// This is set to allow in `[workspace.metadata.cargo-semver-checks.lints]`,
// and since this *package* has the
// `package.metadata.cargo-semver-checks.lints.workspace = true`
// key, this override should be applied.
pub struct StructMissing;
