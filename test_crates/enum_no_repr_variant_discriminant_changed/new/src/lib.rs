#![no_std]

// Explicit discriminant changed values. By doing so, it changed the implicit
// discriminant's value as well. Should be reported.
pub enum ExplicitAndImplicitDiscriminantsAreChanged {
    First = 2,
    Second,
    Third = 5,
}

// Discriminant changed values. Having #[non_exhaustive] on the enum should not have any effect
// on the *API* breakage. Should be reported.
#[non_exhaustive]
pub enum DiscriminantIsChanged {
    First = 1,
}

// Discriminant changed values and the variant is also marked as `non_exhaustive`.
// This now implies that the discriminant is no longer `well-defined`, which means that a numeric
// cast is no longer possible on the enum. Should not be reported.
// https://github.com/rust-lang/reference/pull/1249#issuecomment-2339003824
#[non_exhaustive]
pub enum DiscriminantIsChangedAndMarkedNonExhaustive {
    First,
    #[non_exhaustive]
    Second = 2,
}

// Discriminant changed values, but the variant is already `non_exhaustive`.
// This means that the discriminant is already not `well-defined`, and the numeric cast
// was never possible. Should not be reported.
#[non_exhaustive]
pub enum DiscriminantIsChangedButAlreadyNonExhaustive {
    First,
    #[non_exhaustive]
    Second = 2,
}

// Variant became doc(hidden) while variant became explicit. Being doc(hidden) is not relevant
// since it's still part of the public API. Should be reported.
pub enum DiscriminantBecomesDocHiddenAndExplicit {
    First,
    #[doc(hidden)]
    Second = 2,
}

// This enum starts off not having repr(), then gains repr()
// while also changing discriminant values. This should be reported here:
// the enum originally has no ABI committments so the addition of the repr() doesn't matter.
#[repr(u16)]
pub enum GainsRepr {
    First = 2,
}

// Explicit discriminants changed values, but being private dominates. Should not be
// reported.
enum PrivateEnum {
    First = 10,
    Second = 11,
}

// Discriminant changed values, but the enum has a `repr` attr. Should not be reported as it
// involves ABI breakage as well, and this is linted at *API* only breakage.
#[repr(u8)]
pub enum FieldlessWithDiscrimants {
    First = 10,
    Tuple(),
    Struct {},
    Unit,
    Last = 21,
}

// The unit variant's numeric discriminant changes from 2 to 5 and should be reported.
// Adding repr does not change this: the baseline enum made no ABI commitment.
#[repr(u8)]
pub enum FieldlessChangesToExplicitDescriminant {
    Tuple(),
    Struct {},
    Unit = 5,
}

// Adding a field prevents numeric casts even though the new integer repr exposes
// the changed discriminants. Report cast loss, not changed numeric cast results.
#[repr(u8)]
pub enum UnitOnlyBecomesUndefined {
    First = 1,
    Second,
    Struct { a: i64 },
}

// An explicit empty-variant discriminant prevents casts despite having a known value.
// Report cast loss, not changed numeric cast results.
#[repr(u8)]
pub enum EmptyVariantBecomesExplicit {
    Tuple() = 2,
    Struct {},
}

// Discriminant changed values, but the other variant is marked `non_exhaustive`.
// This means that a numeric cast is not possible on the enum. Should not be reported.
#[non_exhaustive]
pub enum DiscriminantIsChangedButAVariantIsNonExhaustive {
    #[non_exhaustive]
    First,
    Second = 2,
}

// Discriminant changed values, and the variant is no longer `non_exhaustive`.
// The fact that the variant is no longer `non_exhaustive` is not relevant, since a numeric cast
// was impossible to begin with. Should not be reported.
pub enum DiscriminantIsChangedAndVariantNoLongerNonExhaustive {
    First,
    Second = 2,
}

// Drop already prevents numeric casts, so changing the value is not a cast regression.
pub enum DiscriminantChangesWithDrop {
    Value = 2,
}

impl core::ops::Drop for DiscriminantChangesWithDrop {
    fn drop(&mut self) {}
}

// Removing Drop makes casts available; their results were not observable in the baseline.
pub enum DiscriminantChangesAndDropRemoved {
    Value = 2,
}

// Removing a hidden Drop impl makes casts available; the changed numeric value
// was not observable through a cast in the baseline.
pub enum DiscriminantChangesAndHiddenDropRemoved {
    Value = 2,
}

// Adding Drop removes casts entirely, rather than changing their numeric results.
pub enum DiscriminantChangesAndDropAdded {
    Value = 2,
}

impl core::ops::Drop for DiscriminantChangesAndDropAdded {
    fn drop(&mut self) {}
}

// A hidden Drop impl still prevents casts. Report the new cast restriction
// instead of the changed numeric value.
pub enum DiscriminantChangesAndHiddenDropAdded {
    Value = 2,
}

#[doc(hidden)]
impl core::ops::Drop for DiscriminantChangesAndHiddenDropAdded {
    fn drop(&mut self) {}
}

// An unrelated trait named Drop does not affect numeric casts.
pub mod shadow_drop {
    pub trait Drop {}

    pub enum DiscriminantChanges {
        Value = 2,
    }

    impl Drop for DiscriminantChanges {}
}
