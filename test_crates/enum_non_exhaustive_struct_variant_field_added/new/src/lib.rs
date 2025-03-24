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
// That should trigger a separate lint, and not this one.
pub enum EnumWithNonExhaustiveVariant {
    NonExhaustiveVariant { x: i64, y: usize },
}
