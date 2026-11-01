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
// To see this, run the following doctest:
/// ```rust
/// fn example(value: enum_discriminant_no_longer_defined::GainsNonExhaustiveVariant) {
///     value as isize;
/// }
/// ```
#[non_exhaustive]
pub enum GainsNonExhaustiveVariant {
    First,
    Second,
}

/// This shouldn't be reported: this enum's discriminants were not well-defined to begin with
/// because of the non-unit variant.
#[non_exhaustive]
pub enum NonUnitVariantButGainsNonExhaustiveVariant {
    First,
    Second(u16),
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
// To see this, run the following doctest:
/// ```rust
/// fn example(value: enum_discriminant_no_longer_defined::GainsTupleVariant) {
///     value as isize;
/// }
/// ```
#[non_exhaustive]
pub enum GainsTupleVariant {
    None,
    Never,
}

// Same as above, just with a struct variant instead of a tuple variant.
pub enum GainsStructVariant {
    None,
}

// Empty tuple and struct variants do not prevent numeric casts.
#[non_exhaustive]
pub enum GainsEmptyVariants {
    Unit,
}

// An integer repr does not prevent casts when the empty variants have implicit discriminants.
#[non_exhaustive]
pub enum GainsReprAndEmptyVariants {
    Unit,
}

// An explicit discriminant on a unit variant still permits casts of a fieldless enum.
#[non_exhaustive]
pub enum GainsExplicitUnitDiscriminantAndEmptyVariants {
    Unit,
}

// Report once even when multiple doc-hidden non-exhaustive variants are added.
#[non_exhaustive]
pub enum GainsHiddenNonExhaustiveVariants {
    Unit,
}

// Empty variants still prevent downstream casts when they are non-exhaustive.
#[non_exhaustive]
pub enum GainsNonExhaustiveEmptyStructVariant {
    Unit,
}

// Explicit discriminants on empty variants prevent numeric casts.
// When the variant is also non-exhaustive, report both causes of lost casts.
#[non_exhaustive]
pub enum GainsExplicitEmptyVariant {
    Unit,
}

#[non_exhaustive]
pub enum GainsExplicitEmptyStructVariant {
    Unit,
}

// An existing Drop implementation already prevents casts,
// so adding a variant with fields cannot break them.
#[non_exhaustive]
pub enum DropGainsTupleVariant {
    Unit,
}

impl core::ops::Drop for DropGainsTupleVariant {
    fn drop(&mut self) {}
}

// Removing Drop does not make casts part of the baseline API.
#[non_exhaustive]
pub enum RemovedDropGainsStructVariant {
    Unit,
}

impl core::ops::Drop for RemovedDropGainsStructVariant {
    fn drop(&mut self) {}
}

// Adding a variant with fields breaks baseline casts even when Drop is also added.
#[non_exhaustive]
pub enum AddedDropGainsTupleVariant {
    Unit,
}

// An existing Drop implementation also rules out cast breakage from non-exhaustive variants.
#[non_exhaustive]
pub enum DropGainsNonExhaustiveVariant {
    Unit,
}

impl core::ops::Drop for DropGainsNonExhaustiveVariant {
    fn drop(&mut self) {}
}

#[non_exhaustive]
pub enum RemovedDropGainsNonExhaustiveVariant {
    Unit,
}

impl core::ops::Drop for RemovedDropGainsNonExhaustiveVariant {
    fn drop(&mut self) {}
}

// A new non-exhaustive variant breaks baseline casts even when Drop is also added.
#[non_exhaustive]
pub enum AddedDropGainsNonExhaustiveVariant {
    Unit,
}

// The empty variants do not prevent casts; only the new Drop implementation does.
#[non_exhaustive]
pub enum AddedDropGainsEmptyVariants {
    Unit,
}

// An unrelated trait named Drop does not prevent casts of the baseline enums.
pub mod shadow_drop {
    pub trait Drop {}

    #[non_exhaustive]
    pub enum GainsTupleVariant {
        Unit,
    }

    impl Drop for GainsTupleVariant {}

    #[non_exhaustive]
    pub enum GainsNonExhaustiveVariant {
        Unit,
    }

    impl Drop for GainsNonExhaustiveVariant {}
}

// Empty tuple and struct variants permit casts in the baseline enums.
// Adding a variant with fields must report loss of numeric casts.
#[non_exhaustive]
pub enum EmptyTupleGainsFieldfulVariant {
    Empty(),
}

#[non_exhaustive]
pub enum EmptyStructGainsFieldfulVariant {
    Empty {},
}

// With no fields or explicit tuple/struct discriminants, report cast loss
// through the non-exhaustive-variant lint.
#[non_exhaustive]
pub enum EmptyTupleGainsNonExhaustiveVariant {
    Empty(),
}

#[non_exhaustive]
pub enum EmptyStructGainsNonExhaustiveVariant {
    Empty {},
}

// A fieldful non-exhaustive variant disables casts for both reasons.
// Report both causes because both must be removed to restore casts.
#[non_exhaustive]
pub enum EmptyVariantsGainNonExhaustiveFieldfulVariant {
    Tuple(),
    Struct {},
}
