pub enum PubEnum {
    Foo {
        x: i64,
    }
}

// This enum is not public, so it should not be reported by the lint,
// regardless of the new field in the variant.
pub(crate) enum PrivateEnum {
    Foo {
        x: i64,
    }
}

// This enum's variant is non-exhaustive, so it can't be externally constructed
// and can't be matched on without a `..`, and therefore should not be reported by the lint,
// regardless of the new field in the variant.
pub enum EnumWithNonExhaustiveVariant {
    #[non_exhaustive]
    Variant {
        x: i64,
    }
}

// This enum's variant will become non-exhaustive while also gaining a field.
// That should trigger a separate lint, and not this one.
pub enum EnumWithExhaustiveVariant {
    ExhaustiveVariant {
        x: i64,
    }
}
