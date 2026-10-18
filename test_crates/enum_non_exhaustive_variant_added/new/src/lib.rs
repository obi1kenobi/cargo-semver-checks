#![no_std]

// Gains a #[non_exhaustive] variant of each kind, plus a #[doc(hidden)] one.
// The three public variants should be reported, the hidden one should not.
#[non_exhaustive]
pub enum NonExhaustiveEnum {
    Existing(i64),
    #[non_exhaustive]
    AddedPlain,
    #[non_exhaustive]
    AddedTuple(i64),
    #[non_exhaustive]
    AddedStruct { field: i64 },
    #[doc(hidden)]
    #[non_exhaustive]
    AddedHidden,
}

// New #[non_exhaustive] variants should be reported even when the enum
// itself loses #[non_exhaustive].
pub enum BecomesExhaustive {
    Existing(i64),
    #[non_exhaustive]
    AddedPlain,
    #[non_exhaustive]
    AddedTuple(i64),
    #[non_exhaustive]
    AddedStruct {
        field: i64,
    },
}

// Exhaustive enum gaining a #[non_exhaustive] variant: that is a major change
// reported by `enum_variant_added`, so this lint should not report it.
pub enum ExhaustiveEnum {
    Existing(i64),
    #[non_exhaustive]
    Added,
}

// The new variant is not #[non_exhaustive], so this lint should not report it.
#[non_exhaustive]
pub enum GainsExhaustiveVariant {
    Existing(i64),
    Added,
}

// An existing variant becomes #[non_exhaustive]: no variant is added,
// so this lint should not report it.
#[non_exhaustive]
pub enum ExistingVariantMarkedNonExhaustive {
    Existing(i64),
    #[non_exhaustive]
    Marked,
}

// The baseline enum is exhaustive, so `enum_variant_added` reports the new variants.
// `enum_marked_non_exhaustive` separately reports the added attribute.
#[non_exhaustive]
pub enum BecomesNonExhaustive {
    Existing(i64),
    #[non_exhaustive]
    Added,
    #[non_exhaustive]
    AddedTuple(i64),
    #[non_exhaustive]
    AddedStruct {
        field: i64,
    },
}

// Not public API, so this lint should not report it.
#[doc(hidden)]
#[non_exhaustive]
pub enum HiddenEnum {
    Existing(i64),
    #[non_exhaustive]
    Added,
}

mod private {
    // Not importable from outside the crate, so this lint should not report it.
    #[non_exhaustive]
    pub enum PrivateEnum {
        Existing(i64),
        #[non_exhaustive]
        Added,
    }
}
