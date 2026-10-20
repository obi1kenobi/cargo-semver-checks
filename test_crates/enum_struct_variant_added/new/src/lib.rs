#![no_std]

// Gains a struct variant without #[non_exhaustive], which should be reported.
// Also gains variants this lint should not report: a #[non_exhaustive] struct variant
// (reported by `enum_non_exhaustive_variant_added`), unit and tuple variants,
// and a #[doc(hidden)] struct variant.
#[non_exhaustive]
pub enum NonExhaustiveEnum {
    Existing(i64),
    AddedStruct { field: i64 },
    #[non_exhaustive]
    AddedNonExhaustiveStruct { field: i64 },
    AddedUnit,
    AddedTuple(i64),
    #[doc(hidden)]
    AddedHiddenStruct { field: i64 },
}

// A new struct variant should be reported even when the enum also loses #[non_exhaustive].
pub enum BecomesExhaustive {
    Existing(i64),
    AddedStruct { field: i64 },
}

// An existing tuple variant becomes a struct variant. No variant is added,
// so this lint should not report it.
#[non_exhaustive]
pub enum VariantChangesKind {
    Existing(i64),
    Changing { field: i64 },
}

// Exhaustive enum gaining a struct variant: that is a major change
// reported by `enum_variant_added`, so this lint should not report it.
pub enum ExhaustiveEnum {
    Existing(i64),
    Added { field: i64 },
}

// The baseline enum is exhaustive, so `enum_variant_added` reports the new variant.
// `enum_marked_non_exhaustive` separately reports the added attribute.
#[non_exhaustive]
pub enum BecomesNonExhaustive {
    Existing(i64),
    Added { field: i64 },
}

// Not public API, so this lint should not report it.
#[doc(hidden)]
#[non_exhaustive]
pub enum HiddenEnum {
    Existing(i64),
    Added { field: i64 },
}

mod private {
    // Not importable from outside the crate, so this lint should not report it.
    #[non_exhaustive]
    pub enum PrivateEnum {
        Existing(i64),
        Added { field: i64 },
    }
}
