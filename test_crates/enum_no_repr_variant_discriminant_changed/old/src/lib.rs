#![no_std]

// Explicit discriminant changed values. By doing so, it changed the implicit
// discriminant's value as well. Should be reported.
pub enum ExplicitAndImplicitDiscriminantsAreChanged {
    First = 1,
    Second,
    Third = 5,
}

// Discriminant changed values. Having #[non_exhaustive] on the enum should not have any effect
// on the *API* breakage. Should be reported.
#[non_exhaustive]
pub enum DiscriminantIsChanged {
    First,
}

// Discriminant changed values and the variant is also marked as `non_exhaustive`.
// This now implies that the discriminant is no longer `well-defined`, which means that a numeric
// cast is no longer possible on the enum. Should not be reported.
// https://github.com/rust-lang/reference/pull/1249#issuecomment-2339003824
#[non_exhaustive]
pub enum DiscriminantIsChangedAndMarkedNonExhaustive {
    First,
    Second,
}

// Discriminant changed values, but the variant is already `non_exhaustive`.
// This means that the discriminant is already not `well-defined`, and the numeric cast
// was never possible. Should not be reported.
#[non_exhaustive]
pub enum DiscriminantIsChangedButAlreadyNonExhaustive {
    First,
    #[non_exhaustive]
    Second,
}

// Variant became doc(hidden) while variant became explicit. Being doc(hidden) is not relevant
// since it's still part of the public API. Should be reported.
pub enum DiscriminantBecomesDocHiddenAndExplicit {
    First,
    Second,
}

// This enum starts off not having repr(), then gains repr()
// while also changing discriminant values. This should be reported here:
// the enum originally has no ABI committments so the addition of the repr() doesn't matter.
pub enum GainsRepr {
    First = 1,
}

// Explicit discriminants changed values, but being private dominates. Should not be
// reported.
enum PrivateEnum {
    First = 1,
    Second = 2,
}

// Discriminant changed values, but the enum has a `repr` attr. Should not be reported as it
// involves ABI breakage as well, and this is linted at *API* only breakage.
#[repr(u8)]
pub enum FieldlessWithDiscrimants {
    First = 10,
    Tuple(),
    Struct {},
    Unit,
    Last = 20,
}

// The unit variant's numeric discriminant changes from 2 to 5 and should be reported.
// Adding repr does not change this: the baseline enum made no ABI commitment.
pub enum FieldlessChangesToExplicitDescriminant {
    Tuple(),
    Struct {},
    Unit,
}

// Adding a field prevents numeric casts even though the new integer repr exposes
// the changed discriminants. Report cast loss, not changed numeric cast results.
pub enum UnitOnlyBecomesUndefined {
    First,
    Second,
    Struct {},
}

// An explicit empty-variant discriminant prevents casts despite having a known value.
// Report cast loss, not changed numeric cast results.
pub enum EmptyVariantBecomesExplicit {
    Tuple(),
    Struct {},
}

// Discriminant changed values, but the other variant is marked `non_exhaustive`.
// This means that a numeric cast is not possible on the enum. Should not be reported.
#[non_exhaustive]
pub enum DiscriminantIsChangedButAVariantIsNonExhaustive {
    #[non_exhaustive]
    First,
    Second,
}

// Discriminant changed values, and the variant is no longer `non_exhaustive`.
// The fact that the variant is no longer `non_exhaustive` is not relevant, since a numeric cast
// was impossible to begin with. Should not be reported.
pub enum DiscriminantIsChangedAndVariantNoLongerNonExhaustive {
    #[non_exhaustive]
    First,
    Second,
}

// Drop already prevents numeric casts, so changing the value is not a cast regression.
pub enum DiscriminantChangesWithDrop {
    Value = 1,
}

impl core::ops::Drop for DiscriminantChangesWithDrop {
    fn drop(&mut self) {}
}

// Removing Drop makes casts available; their results were not observable in the baseline.
pub enum DiscriminantChangesAndDropRemoved {
    Value = 1,
}

impl core::ops::Drop for DiscriminantChangesAndDropRemoved {
    fn drop(&mut self) {}
}

// Removing a hidden Drop impl makes casts available; the changed numeric value
// was not observable through a cast in the baseline.
pub enum DiscriminantChangesAndHiddenDropRemoved {
    Value = 1,
}

#[doc(hidden)]
impl core::ops::Drop for DiscriminantChangesAndHiddenDropRemoved {
    fn drop(&mut self) {}
}

// Adding Drop removes casts entirely, rather than changing their numeric results.
pub enum DiscriminantChangesAndDropAdded {
    Value = 1,
}

// A hidden Drop impl still prevents casts. Report the new cast restriction
// instead of the changed numeric value.
pub enum DiscriminantChangesAndHiddenDropAdded {
    Value = 1,
}

// An unrelated trait named Drop does not affect numeric casts.
pub mod shadow_drop {
    pub trait Drop {}

    pub enum DiscriminantChanges {
        Value = 1,
    }

    impl Drop for DiscriminantChanges {}
}
