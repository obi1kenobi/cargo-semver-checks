#![no_std]

// Existing struct shouldn't trigger the lint
pub struct ExistingStruct {
    bar: u64,
}
