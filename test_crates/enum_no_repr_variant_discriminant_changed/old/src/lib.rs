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

// Implicit discriminant value changed by becoming explicit. Should not be reported as it involves
// ABI breakage as well, and this is linted at *API* only breakage.
pub enum FieldlessChangesToExplicitDescriminant {
    Tuple(),
    Struct {},
    Unit,
}

// Variant `Struct` acquires a field. By doing so, it makes the discriminant not `well-defined`,
// meaning it is not possible to do a numeric cast anymore on the enum. Should not be reported.
pub enum UnitOnlyBecomesUndefined {
    First,
    Second,
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
