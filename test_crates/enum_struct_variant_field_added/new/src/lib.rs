#![no_std]

pub enum PubEnum {
    Foo {
        x: i64,
        y: usize,
    }
}

// This enum is not public, so it should not be reported by the lint,
// regardless of the new field in the variant.
pub(crate) enum PrivateEnum {
    Foo {
        x: i64,
        y: usize,
    }
}

// This enum's variant is non-exhaustive, so it can't be externally constructed
// and can't be matched on without a `..`, and therefore should not be reported by the lint,
// regardless of the new field in the variant.
pub enum EnumWithNonExhaustiveVariant {
    #[non_exhaustive]
    Variant {
        x: i64,
        y: usize,
    }
}

// This enum's variant will become non-exhaustive while also gaining a field.
// Both the field addition and the new #[non_exhaustive] should be reported.
pub enum EnumWithExhaustiveVariant {
    #[non_exhaustive]
    ExhaustiveVariant {
        x: i64,
        y: usize,
    }
}
