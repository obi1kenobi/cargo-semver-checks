#![no_std]

// Existing struct shouldn't trigger the lint
pub struct ExistingStructWithPrivateField {
    pub bar: u64,
    foo: u64,
}
