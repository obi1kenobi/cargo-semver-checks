#![no_std]

pub enum ExhaustiveEnum {
    #[non_exhaustive]
    Variant,
}

#[non_exhaustive]
pub enum NonExhaustiveEnum {
    #[non_exhaustive]
    Variant,
}

// This enum is private and should not trigger the lint
enum PrivateEnum {
    #[non_exhaustive]
    Variant,
}
