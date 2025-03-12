#![no_std]

pub enum ExhaustiveEnum {
    #[non_exhaustive]
    VariantA,
    VariantB,
}

// This enum is already non-exhaustive
// Changes to its variants should not trigger the lint
#[non_exhaustive]
pub enum NonExhaustiveEnum {
    #[non_exhaustive]
    Variant1,
    Variant2,
}

// This enum is private and should not trigger the lint
enum PrivateEnum {
    #[non_exhaustive]
    VariantX,
    VariantY,
}