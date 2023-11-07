/// The relationship between semver and removing struct fields is complex.
///
/// Removing hidden fields from a plain struct (whether hidden or not) is not a breaking change.
/// Removing fields from hidden plain struct is not breaking, even if the fields
/// themselves weren't marked as hidden. (Plain structs are the ones with curly braces.)
///
/// For tuple structs, the same rules as above apply, with one caveat: if the tuple struct
/// is not itself hidden, the removed fields must not cause non-hidden fields to get renumbered.
/// This is because their index is public API and they may be accessed with `value.1` syntax.
pub mod removals {
    #[doc(hidden)]
    pub struct HiddenPlainStruct {
        x: i64,

        z: i64,
    }

    pub struct VisiblePlainStruct {
        x: i64,
    }

    #[doc(hidden)]
    pub struct HiddenTupleStruct(i64, String);

    pub struct VisibleTupleStructNoReordering(i64);

    /// The removal of the hidden field makes the fields behind it have different indexes.
    pub struct VisibleTupleStructBreaking(i64, String, usize);
}

/// Adding fields to an exhaustive public API struct is always breaking,
/// even if the new fields are hidden from the public API.
pub mod additions {
    pub struct PlainStruct {
        x: i64,

        #[doc(hidden)]
        y: i64,
    }

    pub struct TupleStruct(i64, #[doc(hidden)] i64);

    #[non_exhaustive]
    pub struct NonExhaustivePlainStruct {
        x: i64,

        #[doc(hidden)]
        y: i64,
    }

    #[non_exhaustive]
    pub struct NonExhaustiveTupleStruct(i64, #[doc(hidden)] i64);
}
