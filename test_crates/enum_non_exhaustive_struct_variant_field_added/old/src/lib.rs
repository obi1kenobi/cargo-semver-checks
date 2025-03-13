#![no_std]

pub enum PubEnum {
    #[non_exhaustive]
    Foo {
        x: i64,
    }
}

// This enum is not public, so it should not be reported by the lint,
// regardless of the new field in the variant.
pub(crate) enum PrivateEnum {
    #[non_exhaustive]
    Foo {
        x: i64,
    }
}

// This enum's variant will become exhaustive while also gaining a field.
// That should trigger a separate lint, and not this one.
pub enum EnumWithNonExhaustiveVariant {
    #[non_exhaustive]
    NonExhaustiveVariant {
        x: i64,
    }
}
