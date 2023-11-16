//! Pretending that `#[doc(hidden)]` items simply don't exist is not a valid solution.
//! This crate presents a counterexample: an edge case that would trigger
//! a false-positive "enum_variant_added" lint error with such an implementation.

// In the new version, we make `Sneaky` no longer be `#[doc(hidden)]`.
// This is an *additive-only* change to the public API, not a breaking change.
//
// However, if the `cargo-semver-checks` implementation pretended that hidden items
// simply don't exist, then this change would (incorrectly) look like a breaking change:
// it would appear as an addition of a new variant to an exhaustive enum.
pub enum Example {
    Regular,

    #[doc(hidden)]
    Sneaky,
}
