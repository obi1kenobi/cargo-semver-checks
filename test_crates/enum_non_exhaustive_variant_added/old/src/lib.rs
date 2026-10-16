#![no_std]

// Gains a #[non_exhaustive] variant of each kind, plus a #[doc(hidden)] one.
// The three public variants should be reported, the hidden one should not.
#[non_exhaustive]
pub enum NonExhaustiveEnum {
    Existing(i64),
}

// Exhaustive enum gaining a #[non_exhaustive] variant: that is a major change
// reported by `enum_variant_added`, so this lint should not report it.
pub enum ExhaustiveEnum {
    Existing(i64),
}

// The new variant is not #[non_exhaustive], so this lint should not report it.
#[non_exhaustive]
pub enum GainsExhaustiveVariant {
    Existing(i64),
}

// An existing variant becomes #[non_exhaustive]: no variant is added,
// so this lint should not report it.
#[non_exhaustive]
pub enum ExistingVariantMarkedNonExhaustive {
    Existing(i64),
    Marked,
}

// The enum only becomes #[non_exhaustive] in the new version, which
// `enum_marked_non_exhaustive` reports, so this lint should not report it.
pub enum BecomesNonExhaustive {
    Existing(i64),
}

// Not public API, so this lint should not report it.
#[doc(hidden)]
#[non_exhaustive]
pub enum HiddenEnum {
    Existing(i64),
}

mod private {
    // Not importable from outside the crate, so this lint should not report it.
    #[non_exhaustive]
    pub enum PrivateEnum {
        Existing(i64),
    }
}
