#![no_std]

// Explicit discriminant changed values. By doing so, it changed the implicit
// discriminant's value as well. Should be reported.
#[repr(isize)]
pub enum ExplicitAndImplicitDiscriminantsAreChanged {
    First = 1,
    Second,
    Third = 5,
}

// Discriminant changed values. Having #[non_exhaustive] on the enum should not have any effect
// on the API or ABI breakage. Should be reported.
#[repr(i16)]
#[non_exhaustive]
pub enum DiscriminantIsChanged {
    First,
}

// Discriminant changed values, while the other variant is marked `non_exhaustive`.
// The variant's exhaustiveness is not relevant for pointer casting, so this should be reported.
#[non_exhaustive]
#[repr(u32)]
pub enum DiscriminantIsChangedButAVariantIsNonExhaustive {
    #[non_exhaustive]
    First,
    Second,
}
