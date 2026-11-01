#![no_std]

// Keep integer representations and existing discriminant values unchanged so these
// cases isolate the loss of numeric casts from changes to numeric values.

#[repr(u8)]
pub enum EmptyTupleGainsExplicitDiscriminant {
    Variant(),
}

// An unrelated trait named Drop does not prevent casts of the baseline enum.
mod shadow_drop {
    pub trait Drop {}
}
impl shadow_drop::Drop for EmptyTupleGainsExplicitDiscriminant {}

// Hiding a variant from rustdoc does not hide its effect on the enum's castability.
#[repr(u8)]
pub enum HiddenEmptyStructGainsExplicitDiscriminant {
    #[doc(hidden)]
    Variant {},
}

#[repr(u8)]
pub enum ExplicitUnitBecomesTuple {
    Variant = 0,
}

#[repr(u8)]
pub enum ExplicitUnitBecomesStruct {
    Variant = 0,
}

#[repr(u8)]
pub enum MultipleVariantsGainExplicitDiscriminants {
    Tuple(),
    Struct {},
}

#[repr(u8)]
#[non_exhaustive]
pub enum ExplicitUnitAddedAlongsideEmptyVariants {
    Tuple(),
    Struct {},
}

#[repr(u8)]
pub enum TupleGainsOnlyFields {
    Variant(),
}

#[repr(u8)]
pub enum StructGainsOnlyFields {
    Variant {},
}

// The following enums already forbid numeric casts in downstream code.
// Removing that cause while introducing another must not report a lost capability.
// Hiding the variant, field, or implementation does not remove that restriction.
#[repr(u8)]
pub enum BaselineHasFields {
    #[doc(hidden)]
    Fields(#[doc(hidden)] u8),
    Explicit(),
}

#[repr(u8)]
pub enum BaselineHasStructFields {
    Fields { value: u8 },
    Explicit(),
}

#[repr(u8)]
pub enum BaselineHasExplicitNonUnitDiscriminant {
    First() = 0,
    Second {},
}

#[repr(u8)]
pub enum BaselineHasDrop {
    Variant(),
}

#[doc(hidden)]
impl Drop for BaselineHasDrop {
    fn drop(&mut self) {}
}

#[repr(u8)]
pub enum BaselineHasNonExhaustiveVariant {
    #[doc(hidden)]
    #[non_exhaustive]
    Variant {},
}

#[repr(u8)]
enum PrivateEnum {
    Variant(),
}

#[repr(u8)]
#[doc(hidden)]
pub enum HiddenEnum {
    Variant(),
}

#[repr(u8)]
#[doc(hidden)]
pub enum PreviouslyHiddenEnum {
    Variant(),
}

#[repr(u8)]
pub enum BecomesHiddenEnum {
    Variant(),
}

mod unreachable {
    #[repr(u8)]
    pub enum UnreachableEnum {
        Variant(),
    }
}

mod implementation {
    #[repr(u8)]
    pub enum Before {
        Variant(),
    }
}

pub use implementation::Before as ReexportedEnum;
