#![no_std]

// Existing exhaustive struct shouldn't trigger the lint
pub struct ExistingStruct {
    pub bar: u64,
}

// Existing #[non_exhaustive] struct shouldn't trigger the lint
#[non_exhaustive]
pub struct ExistingNonExhaustiveStruct {
    pub bar: u64,
}
