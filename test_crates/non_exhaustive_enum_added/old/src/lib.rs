#![no_std]

// Existing enum shouldn't trigger the lint
#[non_exhaustive]
pub enum ExistingEnum {
    VariantA,
}
