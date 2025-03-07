#![no_std]

// This will be removed in the `new` version to trigger the `struct_missing` lint.
// This is configured to `allow` in `old/Cargo.toml` (the baseline version),
// but this should not affect the lint level as the configuration should only be
// read from the `new` (current) manifest, so the `struct_missing` lint should
// still trigger.
pub struct StructMissing;
