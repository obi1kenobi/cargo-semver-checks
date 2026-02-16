#![no_std]

// Existing struct shouldn't trigger the lint
pub struct ExistingStruct {
    pub bar: u64,
}
