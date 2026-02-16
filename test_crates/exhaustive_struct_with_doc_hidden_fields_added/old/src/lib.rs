#![no_std]

// Existing struct shouldn't trigger the lint
pub struct ExistingStructWithHiddenField {
    pub bar: u64,
    #[doc(hidden)]
    pub foo: u64,
}
