// Explicit discriminant changed values. By doing so, it changed the implicit
// discriminant's value as well, should be reported.
#[repr(u8, C)]
pub enum ExplicitAndImplicitDiscriminantsAreChanged {
    First = 1,
    Second,
    Third = 5,
}

// Implicit discriminant changed values when becoming explicit, should be reported.
#[repr(u8)]
pub enum ImplicitDiscriminantBecomesExplicit {
    Tuple(),
    Struct {},
    Unit,
}

// Discriminant changed to be doc hidden and explicit. Being doc hidden is not relevant
// since it's still part of the public API, should be reported.
pub enum DiscriminantBecomesDocHiddenAndExplicit {
    First,
    Second,
}

// Explicit discriminants changed values, but being private dominates, should not be
// reported.
enum PrivateEnum {
    First = 1,
    Second = 2,
}
