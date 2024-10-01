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

// Implicit discriminant value changed by becoming explicit. Should not be reported as it involves
// ABI breakage as well, and this is linted at *API* only breakage.
#[repr(u8)]
pub enum FieldlessChangesToExplicitDescriminant {
    Tuple(),
    Struct {},
    Unit = 5,
}

// Variant `Struct` acquires a field. By doing so, it makes the discriminant not `well-defined`,
// meaning it is not possible to do a numeric cast anymore on the enum. Should not be reported.
pub enum UnitOnlyBecomesUndefined {
    First,
    Second,
    Struct { a: i64 },
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
