#![no_std]

// These fieldless enums permit numeric casts despite having tuple and struct variants.
// The new crate tests every combination of four ways to lose that ability:
// Drop, fields, non-exhaustive variants, and explicit non-unit discriminants.
// All enums are non-exhaustive so adding variants does not itself break matching.
// Representative C and integer repr cases ensure all causes are reported
// even when the enum already has an explicit representation.

#[non_exhaustive]
pub enum StillCastable {
    Tuple(),
    Struct {},
}

#[non_exhaustive]
pub enum DropOnly {
    Tuple(),
    Struct {},
}

#[non_exhaustive]
pub enum FieldsOnly {
    Tuple(),
    Struct {},
}

#[non_exhaustive]
pub enum NonExhaustiveOnly {
    Tuple(),
    Struct {},
}

#[non_exhaustive]
pub enum ExplicitOnly {
    Tuple(),
    Struct {},
}

#[non_exhaustive]
pub enum DropAndFields {
    Tuple(),
    Struct {},
}

#[non_exhaustive]
pub enum DropAndNonExhaustive {
    Tuple(),
    Struct {},
}

#[non_exhaustive]
pub enum DropAndExplicit {
    Tuple(),
    Struct {},
}

#[repr(C)]
#[non_exhaustive]
pub enum FieldsAndNonExhaustive {
    Tuple(),
    Struct {},
}

#[non_exhaustive]
pub enum FieldsAndExplicit {
    Tuple(),
    Struct {},
}

#[non_exhaustive]
pub enum NonExhaustiveAndExplicit {
    Tuple(),
    Struct {},
}

#[repr(u8)]
#[non_exhaustive]
pub enum DropFieldsAndNonExhaustive {
    Tuple(),
    Struct {},
}

#[non_exhaustive]
pub enum DropFieldsAndExplicit {
    Tuple(),
    Struct {},
}

#[non_exhaustive]
pub enum DropNonExhaustiveAndExplicit {
    Tuple(),
    Struct {},
}

#[non_exhaustive]
pub enum FieldsNonExhaustiveAndExplicit {
    Tuple(),
    Struct {},
}

// An explicit unit discriminant must not prevent any lint from recognizing baseline casts.
#[repr(u8)]
#[non_exhaustive]
pub enum AllCauses {
    Tuple(),
    Struct {},
    Unit = 5,
}

#[non_exhaustive]
pub enum FieldsAndExplicitSameTupleVariant {
    Tuple(),
    Struct {},
}

#[non_exhaustive]
pub enum AllCausesSameStructVariant {
    Tuple(),
    Struct {},
}
