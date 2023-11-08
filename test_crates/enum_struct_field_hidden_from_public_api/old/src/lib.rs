/// The relationship between semver and removing fields from enum variants is complex.
///
/// Removing hidden fields from a struct variant (whether hidden or not) is not a breaking change.
/// Removing fields from hidden struct variants is not breaking, even if the fields
/// themselves weren't marked as hidden.
///
/// For tuple variants, the same rules as above apply, with one caveat: if the tuple variant
/// is not itself hidden, the removed fields must not cause non-hidden fields to get renumbered.
/// This is because their index is public API and they may be accessed with `variant.1` syntax.
pub enum RemovedHiddenFieldFromVariant {
    Other,

    #[doc(hidden)]
    HiddenStructVariant {
        x: i64,

        #[doc(hidden)]
        y: i64,

        z: i64,
    },

    VisibleStructVariant {
        x: i64,

        #[doc(hidden)]
        y: i64,
    },

    #[doc(hidden)]
    HiddenTupleVariant(i64, #[doc(hidden)] i64, String),

    VisibleTupleVariantNoReordering(i64, #[doc(hidden)] i64),

    /// The removal of the hidden field makes the fields behind it have different indexes.
    VisibleTupleVariantBreaking(i64, #[doc(hidden)] i64, String, usize),
}

/// Adding fields to an exhaustive public API variant of a public API enum is always breaking,
/// even if the new fields are hidden from the public API.
pub enum AddedVariantField {
    StructVariant {
        x: i64,
    },

    TupleVariant(i64),

    #[non_exhaustive]
    NonExhaustiveStructVariant {
        x: i64,
    },

    #[non_exhaustive]
    NonExhaustiveTupleVariant(i64),
}
