#![no_std]

pub enum ExhaustiveEnum {
    #[non_exhaustive]
    Foo { x: i64, y: usize },
}

#[non_exhaustive]
pub enum NonExhaustiveEnum {
    #[non_exhaustive]
    Foo { x: i64, y: usize },
}

// This enum is not public, so it should not be reported by the lint,
// regardless of the new field in the variant.
pub(crate) enum PrivateEnum {
    #[non_exhaustive]
    Foo { x: i64, y: usize },
}

// This enum's variant will become exhaustive while also gaining a field.
// It should trigger this lint.
pub enum EnumWithNonExhaustiveVariant {
    NonExhaustiveVariant { x: i64, y: usize },
}
