#![no_std]

// Existing struct should not trigger the lint
pub struct ExistingStruct {
    pub bar: u64,
}

#[non_exhaustive]
pub struct ExistingNonExhaustiveStruct {
    pub bar: u64,
}
// New non-exhaustive struct with public fields should trigger the lint
#[non_exhaustive]
pub struct NewNonExhaustiveStruct {
    pub bar: u64,
}

// This #[non_exhaustive] struct is not public, so it should not trigger the lint.
struct PrivateStruct {
    foo: u64,
}

// This #[non_exhaustive] struct have non-public fields , so it should not trigger the lint.
#[non_exhaustive]
pub struct StructWithPrivateField {
    foo: u64,
}

// This struct is exhaustive, so it should not trigger this lint
pub struct NonExhaustiveStruct {
    bar: u64,
}

// This struct shouldn't trigger the lint because it cannot be constructed
// with a literal within the public API of the crate.
#[non_exhaustive]
pub struct StructWithHiddenField {
    pub a: i64,

    #[doc(hidden)]
    pub b: i64,
}
