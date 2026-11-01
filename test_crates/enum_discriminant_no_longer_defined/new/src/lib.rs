#![no_std]

// It's not allowed to use `as isize` to cast another crate's enum when
// the enum contains a non-exhaustive variant:
//
// ---- src/lib.rs - GainsNonExhaustiveVariant (line 1) stdout ----
// error[E0606]: casting `GainsNonExhaustiveVariant` as `isize` is invalid
//  --> src/lib.rs:4:5
//   |
// 6 |     value as isize;
//   |     ^^^^^^^^^^^^^^
//   |
//   = note: cannot cast an enum with a non-exhaustive variant when it's defined in another crate
//
// error: aborting due to 1 previous error
//
// To see this, remove `compile_fail` and then run the following doctest:
/// ```rust,compile_fail
/// fn example(value: enum_discriminant_no_longer_defined::GainsNonExhaustiveVariant) {
///     value as isize;
/// }
/// ```
#[non_exhaustive]
pub enum GainsNonExhaustiveVariant {
    First,
    Second,
    #[non_exhaustive]  // TODO: this needs to be flagged but currently isn't.
    Third,
}

/// This shouldn't be reported: this enum's discriminants were not well-defined to begin with
/// because of the non-unit variant.
#[non_exhaustive]
pub enum NonUnitVariantButGainsNonExhaustiveVariant {
    First,
    Second(u16),
    #[non_exhaustive]
    Third,
}

// It's not allowed to use `as isize` to cast another crate's enum when
// the enum contains a variant with fields:
//
// ---- src/lib.rs - GainsTupleVariant (line 1) stdout ----
// error[E0605]: non-primitive cast: `GainsTupleVariant` as `isize`
//  --> src/lib.rs:4:5
//   |
// 6 |     value as isize;
//   |     ^^^^^^^^^^^^^^ an `as` expression can be used to convert enum types to numeric types only if the enum type is unit-only or field-less
//   |
//   = note: see https://doc.rust-lang.org/reference/items/enumerations.html#casting for more information
//
// To see this, remove `compile_fail` and then run the following doctest:
/// ```rust,compile_fail
/// fn example(value: enum_discriminant_no_longer_defined::GainsTupleVariant) {
///     value as isize;
/// }
/// ```
#[non_exhaustive]
pub enum GainsTupleVariant {
    None,
    Never,
    Some(core::num::NonZeroUsize),
}

// Same as above, just with a struct variant instead of a tuple variant.
pub enum GainsStructVariant {
    None,
    Some { value: core::num::NonZeroUsize },
}

// Empty tuple and struct variants do not prevent numeric casts.
#[non_exhaustive]
pub enum GainsEmptyVariants {
    Unit,
    Tuple(),
    Struct {},
}

// An integer repr does not prevent casts when the empty variants have implicit discriminants.
#[non_exhaustive]
#[repr(u8)]
pub enum GainsReprAndEmptyVariants {
    Unit,
    Tuple(),
    Struct {},
}

// An explicit discriminant on a unit variant still permits casts of a fieldless enum.
#[non_exhaustive]
#[repr(u8)]
pub enum GainsExplicitUnitDiscriminantAndEmptyVariants {
    Unit = 0,
    Tuple(),
    Struct {},
}

// Report once even when multiple doc-hidden non-exhaustive variants are added.
#[non_exhaustive]
pub enum GainsHiddenNonExhaustiveVariants {
    Unit,
    #[doc(hidden)]
    #[non_exhaustive]
    Tuple(),
    #[doc(hidden)]
    #[non_exhaustive]
    Struct {},
}

// Empty variants still prevent downstream casts when they are non-exhaustive.
#[non_exhaustive]
pub enum GainsNonExhaustiveEmptyStructVariant {
    Unit,
    #[non_exhaustive]
    Struct {},
}

// Explicit discriminants on empty variants prevent numeric casts.
// When the variant is also non-exhaustive, report both causes of lost casts.
#[non_exhaustive]
#[repr(u8)]
pub enum GainsExplicitEmptyVariant {
    Unit,
    #[non_exhaustive]
    Tuple() = 1,
}

#[non_exhaustive]
#[repr(u8)]
pub enum GainsExplicitEmptyStructVariant {
    Unit,
    Struct {} = 1,
}

// An existing Drop implementation already prevents casts,
// so adding a variant with fields cannot break them.
#[non_exhaustive]
pub enum DropGainsTupleVariant {
    Unit,
    Tuple(u8),
}

impl core::ops::Drop for DropGainsTupleVariant {
    fn drop(&mut self) {}
}

// Removing Drop does not make casts part of the baseline API.
#[non_exhaustive]
pub enum RemovedDropGainsStructVariant {
    Unit,
    Struct { value: u8 },
}

// Adding a variant with fields breaks baseline casts even when Drop is also added.
#[non_exhaustive]
pub enum AddedDropGainsTupleVariant {
    Unit,
    Tuple(u8),
}

impl core::ops::Drop for AddedDropGainsTupleVariant {
    fn drop(&mut self) {}
}

// An existing Drop implementation also rules out cast breakage from non-exhaustive variants.
#[non_exhaustive]
pub enum DropGainsNonExhaustiveVariant {
    Unit,
    #[non_exhaustive]
    Added,
}

impl core::ops::Drop for DropGainsNonExhaustiveVariant {
    fn drop(&mut self) {}
}

#[non_exhaustive]
pub enum RemovedDropGainsNonExhaustiveVariant {
    Unit,
    #[non_exhaustive]
    Added,
}

// A new non-exhaustive variant breaks baseline casts even when Drop is also added.
#[non_exhaustive]
pub enum AddedDropGainsNonExhaustiveVariant {
    Unit,
    #[non_exhaustive]
    Added,
}

impl core::ops::Drop for AddedDropGainsNonExhaustiveVariant {
    fn drop(&mut self) {}
}

// The empty variants do not prevent casts; only the new Drop implementation does.
#[non_exhaustive]
pub enum AddedDropGainsEmptyVariants {
    Unit,
    Tuple(),
    Struct {},
}

impl core::ops::Drop for AddedDropGainsEmptyVariants {
    fn drop(&mut self) {}
}

// An unrelated trait named Drop does not prevent casts of the baseline enums.
pub mod shadow_drop {
    pub trait Drop {}

    #[non_exhaustive]
    pub enum GainsTupleVariant {
        Unit,
        Tuple(u8),
    }

    impl Drop for GainsTupleVariant {}

    #[non_exhaustive]
    pub enum GainsNonExhaustiveVariant {
        Unit,
        #[non_exhaustive]
        Added,
    }

    impl Drop for GainsNonExhaustiveVariant {}
}

// Empty tuple and struct variants permit casts in the baseline enums.
// Adding a variant with fields must report loss of numeric casts.
#[non_exhaustive]
pub enum EmptyTupleGainsFieldfulVariant {
    Empty(),
    Added(u8),
}

#[non_exhaustive]
pub enum EmptyStructGainsFieldfulVariant {
    Empty {},
    Added { value: u8 },
}

// With no fields or explicit tuple/struct discriminants, report cast loss
// through the non-exhaustive-variant lint.
#[non_exhaustive]
pub enum EmptyTupleGainsNonExhaustiveVariant {
    Empty(),
    #[non_exhaustive]
    Added,
}

#[non_exhaustive]
pub enum EmptyStructGainsNonExhaustiveVariant {
    Empty {},
    #[non_exhaustive]
    Added,
}

// A fieldful non-exhaustive variant disables casts for both reasons.
// Report both causes because both must be removed to restore casts.
#[non_exhaustive]
pub enum EmptyVariantsGainNonExhaustiveFieldfulVariant {
    Tuple(),
    Struct {},
    #[non_exhaustive]
    Added(u8),
}
