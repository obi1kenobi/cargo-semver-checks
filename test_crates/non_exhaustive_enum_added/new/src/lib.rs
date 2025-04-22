#![no_std]

// Existing enum shouldn't trigger the lint
#[non_exhaustive]
pub enum ExistingEnum {
    VariantA,
}

// This enum is exhaustive, so it should not trigger the lint.
pub enum ExhaustiveEnum {
    VariantA,
}

// This enum is not public, so it should not trigger the lint.
enum PrivateEnum {
    VariantA,
}

#[non_exhaustive]
pub enum NonExhaustiveEnum {
    VariantA,
}
