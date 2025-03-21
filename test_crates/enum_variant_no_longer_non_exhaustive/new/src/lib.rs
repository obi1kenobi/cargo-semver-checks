#![no_std]

pub enum ExhaustiveEnum {
    Variant,
}

#[non_exhaustive]
pub enum NonExhaustiveEnum {
    Variant,
}

// This enum is private and should not trigger the lint
enum PrivateEnum {
    Variant,
}
