#![no_std]

pub enum ExhaustiveEnum {
    #[non_exhaustive]
    TupleVariantWithFieldAdded(i32, i64, u8),
}

#[non_exhaustive]
pub enum NonExhaustiveEnum {
    #[non_exhaustive]
    TupleVariantWithFieldAdded(i32, i64, u8),
}

// Changes in a private enum should not be reported
enum PrivateEnum {
    #[non_exhaustive]
    TupleVariantWithFieldAdded(i32, i64, u8),
}

// This enum's variant will become exhaustive while also gaining a field.
// It should trigger this lint.
pub enum EnumWithNonExhaustiveVariant {
    TupleVariantWithFieldAdded(i32, i64, u8),
}
