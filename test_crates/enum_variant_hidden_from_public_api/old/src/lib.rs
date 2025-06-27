#![no_std]

/// Hiding a variant from the public API is a breaking change.
pub enum HiddenVariant {
    ToBeHidden,
    Other,
}

/// Adding a variant to an exhaustive public API enum is always breaking,
/// even if the new variant is hidden from the public API.
pub enum AddedVariant {
    First,
    Second,
}

/// Removing a hidden variant from a public API enum is not a breaking change.
pub enum RemovedHiddenVariant {
    Other,

    #[doc(hidden)]
    ToBeRemoved,
}

/// Removing fields from a hidden variant of a public API enum is not a breaking change.
pub enum RemovedFieldFromHiddenVariant {
    Other,

    #[doc(hidden)]
    RemovedStructField {
        x: i64,
        y: i64,
    },

    #[doc(hidden)]
    RemovedTupleField(i64, i64),
}
