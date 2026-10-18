#![no_std]

// Gains a unit variant without #[non_exhaustive], which should be reported.
// Also gains variants this lint should not report: a #[non_exhaustive] unit variant
// (reported by `enum_non_exhaustive_variant_added`), tuple and struct variants,
// and a #[doc(hidden)] unit variant.
#[non_exhaustive]
pub enum NonExhaustiveEnum {
    Existing(i64),
    AddedUnit,
    #[non_exhaustive]
    AddedNonExhaustiveUnit,
    AddedTuple(i64),
    AddedStruct { field: i64 },
    #[doc(hidden)]
    AddedHiddenUnit,
}

// A new unit variant should be reported even when the enum also loses #[non_exhaustive].
pub enum BecomesExhaustive {
    Existing(i64),
    AddedUnit,
}

// An existing tuple variant becomes a unit variant. No variant is added,
// so this lint should not report it.
#[non_exhaustive]
pub enum VariantChangesKind {
    Existing(i64),
    Changing,
}

// Exhaustive enum gaining a unit variant: that is a major change
// reported by `enum_variant_added`, so this lint should not report it.
pub enum ExhaustiveEnum {
    Existing(i64),
    Added,
}

// The baseline enum is exhaustive, so `enum_variant_added` reports the new variant.
// `enum_marked_non_exhaustive` separately reports the added attribute.
#[non_exhaustive]
pub enum BecomesNonExhaustive {
    Existing(i64),
    Added,
}

// Not public API, so this lint should not report it.
#[doc(hidden)]
#[non_exhaustive]
pub enum HiddenEnum {
    Existing(i64),
    Added,
}

mod private {
    // Not importable from outside the crate, so this lint should not report it.
    #[non_exhaustive]
    pub enum PrivateEnum {
        Existing(i64),
        Added,
    }
}
