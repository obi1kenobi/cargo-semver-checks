#![no_std]

// Existing struct should not trigger the lint
pub struct ExistingStruct {
    pub bar: u64,
}

// New non-exhaustive struct with public fields should trigger the lint
pub struct NewStruct {
    pub bar : u64,
}

// This struct is not public, so it should not trigger the lint.
struct PrivateStruct {
    foo: u64,
}

// This struct have non-public fields , so it should not trigger the lint.
pub struct StructWithPrivateField {
    foo : u64
}

// This struct is #[non_exhaustive], so it should not trigger this lint
#[non_exhaustive]
pub struct NonExhaustiveStruct {
    bar: u64,
}
