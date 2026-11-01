#![no_std]

// Report explicit non-unit discriminants independently of fields. All existing
// discriminant values and integer representations remain unchanged.

// Should report: an existing empty tuple variant gains an explicit discriminant.
#[repr(u8)]
pub enum EmptyTupleGainsExplicitDiscriminant {
    Variant() = 0,
}

// An unrelated trait named Drop does not prevent casts of the baseline enum.
mod shadow_drop {
    pub trait Drop {}
}
impl shadow_drop::Drop for EmptyTupleGainsExplicitDiscriminant {}

// Should report: a doc-hidden variant still prevents casts of its public enum.
#[repr(u8)]
pub enum HiddenEmptyStructGainsExplicitDiscriminant {
    #[doc(hidden)]
    Variant {} = 0,
}

// Should report: an explicit discriminant is retained on a newly non-unit variant.
#[repr(u8)]
pub enum ExplicitUnitBecomesTuple {
    Variant() = 0,
}

// Should report the struct variant case as well.
#[repr(u8)]
pub enum ExplicitUnitBecomesStruct {
    Variant {} = 0,
}

// Should report this enum once, despite both variants gaining explicit discriminants.
#[repr(u8)]
pub enum MultipleVariantsGainExplicitDiscriminants {
    Tuple() = 0,
    Struct {} = 1,
}

// Should not report: explicit unit discriminants allow casts when non-unit variants
// remain fieldless and have implicit discriminants.
#[repr(u8)]
#[non_exhaustive]
pub enum ExplicitUnitAddedAlongsideEmptyVariants {
    Tuple(),
    Struct {},
    Unit = 2,
}

// The fields discriminant-loss lint should report this enum once despite two new fields;
// the explicit-discriminant lint should not.
#[repr(u8)]
pub enum TupleGainsOnlyFields {
    Variant(u8, u16),
}

// The fields discriminant-loss lint should report even though the new field is doc-hidden;
// the explicit-discriminant lint should not.
#[repr(u8)]
pub enum StructGainsOnlyFields {
    Variant {
        #[doc(hidden)]
        value: u8,
    },
}

// No discriminant-loss lint should report: the baseline's hidden field already forbids casts.
#[repr(u8)]
pub enum BaselineHasFields {
    #[doc(hidden)]
    Fields(),
    #[non_exhaustive]
    Explicit() = 1,
}

// Should not report: the baseline's struct field already forbids casts.
#[repr(u8)]
pub enum BaselineHasStructFields {
    Fields {},
    Explicit() = 1,
}

// Neither discriminant-loss lint should report: the baseline's explicit non-unit
// discriminant already forbids casts, even though it moves and a field is added.
#[repr(u8)]
pub enum BaselineHasExplicitNonUnitDiscriminant {
    First(u8),
    Second {} = 1,
}

// No discriminant-loss lint should report: the baseline's hidden Drop impl already
// forbids casts. Fields, an explicit discriminant, and non-exhaustiveness replace it.
#[repr(u8)]
pub enum BaselineHasDrop {
    #[non_exhaustive]
    Variant(u8) = 0,
}

// Neither discriminant-loss lint should report: the baseline variant's
// non-exhaustiveness already forbids casts, despite its removal in this version.
#[repr(u8)]
pub enum BaselineHasNonExhaustiveVariant {
    #[doc(hidden)]
    Variant { value: u8 } = 0,
}

// Should not report: a private enum is not part of the public API.
#[repr(u8)]
enum PrivateEnum {
    Variant() = 0,
}

// Should not report: a doc-hidden enum is not part of the public API.
#[repr(u8)]
#[doc(hidden)]
pub enum HiddenEnum {
    Variant() = 0,
}

// No discriminant-loss lint should report: the enum was not public API in the baseline.
#[repr(u8)]
pub enum PreviouslyHiddenEnum {
    #[non_exhaustive]
    Variant(u8) = 0,
}

// No discriminant-loss lint should report: the enum is no longer public API.
#[repr(u8)]
#[doc(hidden)]
pub enum BecomesHiddenEnum {
    #[non_exhaustive]
    Variant(u8) = 0,
}

mod unreachable {
    // Should not report: this pub enum has no publicly importable path.
    #[repr(u8)]
    pub enum UnreachableEnum {
        Variant() = 0,
    }
}

mod implementation {
    // Should report through the stable reexport path despite the changed type name.
    #[repr(u8)]
    pub enum After {
        Variant() = 0,
    }
}

pub use implementation::After as ReexportedEnum;
