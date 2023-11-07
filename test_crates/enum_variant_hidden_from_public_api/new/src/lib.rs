/// Hiding a variant from the public API is a breaking change.
pub enum HiddenVariant {
    #[doc(hidden)]
    ToBeHidden,

    Other,
}

/// Adding a variant to an exhaustive public API enum is always breaking,
/// even if the new variant is hidden from the public API.
pub enum AddedVariant {
    First,
    Second,

    #[doc(hidden)]
    Added,
}

/// Removing a hidden variant from a public API enum is not a breaking change.
pub enum RemovedHiddenVariant {
    Other,
}

/// Removing fields from a hidden variant of a public API enum is not a breaking change.
pub enum RemovedFieldFromHiddenVariant {
    Other,

    #[doc(hidden)]
    RemovedStructField {
        x: i64,
    },

    #[doc(hidden)]
    RemovedTupleField(i64),
}
