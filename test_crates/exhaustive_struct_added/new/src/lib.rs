#![no_std]

// Existing struct should not trigger the lint
pub struct ExistingStruct {
    foo: u64,
}

// New non-exhaustive struct should trigger the lint
pub struct NewStruct {
    bar : u64,
}

// This struct is not public, so it should not trigger the lint.
struct PrivateStruct {
    foo: u64,
}

// This enum is #[non_exhaustive], so it should not trigger this lint
#[non_exhaustive]
pub struct NonExhaustiveStruct {
    bar: u64,
}
