#![no_std]

// Existing enum should not trigger the lint
pub enum ExistingEnum {
    VariantA,
}

pub enum NewEnum {
    VariantA,
}

// This enum is not public, so it should not trigger the lint.
enum PrivateEnum {
    VariantA,
}

// This enum is #[non_exhaustive], so it should not trigger this lint
#[non_exhaustive]
pub enum NonExhaustiveEnum {
    VariantA,
}
