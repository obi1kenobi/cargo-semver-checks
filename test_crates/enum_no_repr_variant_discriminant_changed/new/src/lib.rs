// Explicit discriminant changed values. By doing so, it changed the implicit
// discriminant's value as well, should be reported.
pub enum ExplicitAndImplicitDiscriminantsAreChanged {
    First = 2,
    Second,
    Third = 5,
}

// Discriminant changed to be doc hidden and explicit. Being doc hidden is not relevant
// since it's still part of the public API, should be reported.
pub enum DiscriminantBecomesDocHiddenAndExplicit {
    First,
    #[doc(hidden)]
    Second = 2,
}

// Explicit discriminants changed values, but being private dominates, should not be
// reported.
enum PrivateEnum {
    First = 10,
    Second = 11,
}

// Discriminant changed value, but with repr, should not be reported as it involves ABI
// breakage as well, and this is linted at *API* only breakage.
#[repr(u8)]
pub enum FieldlessWithDiscrimants {
    First = 10,
    Tuple(),
    Struct {},
    Unit,
    Last = 21,
}

// Implicit discriminant value changed by becoming explicit, should not be reported as it involves
// ABI breakage as well, and this is linted at *API* only breakage.
#[repr(u8)]
pub enum FieldlessChangesToExplicitDescriminant {
    Tuple(),
    Struct {},
    Unit = 5,
}

// Variant `Struct` acquires a field. By doing so, it makes the enum not `well-defined`,
// meaning it is not possible to do a numeric cast anymore on the enum, should not be reported.
pub enum UnitOnlyBecomesUndefined {
    First,
    Second,
    Struct { a: i64 },
}
