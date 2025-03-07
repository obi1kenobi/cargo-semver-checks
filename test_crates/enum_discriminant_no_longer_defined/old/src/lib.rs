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
// the enum contains a non-unit variant:
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
