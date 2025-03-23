#![no_std]

// Existing enum shouldn't trigger the lint
pub enum ExistingEnum {
    VariantA,
}
