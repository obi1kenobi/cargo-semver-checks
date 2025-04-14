#![no_std]

// This pub enum's variant is no longer #[non_exhaustive], should trigger the lint.
pub enum ExhaustiveEnumUnitVariant {
    UnitVariant,
}

// This pub enum's variant is no longer #[non_exhaustive], should trigger the lint.
#[non_exhaustive]
pub enum NonExhaustiveEnumUnitVariant {
    UnitVariant,
}

// This pub enum's variant is no longer #[non_exhaustive], should trigger the lint.
pub enum ExhaustiveEnumTupleVariant {
    TupleVariant(i32, i64),
}

// This pub enum's variant is no longer #[non_exhaustive], should trigger the lint.
#[non_exhaustive]
pub enum NonExhaustiveEnumTupleVariant {
    TupleVariant(i32, i64),
}

// This pub enum's variant is no longer #[non_exhaustive], should trigger the lint.
pub enum ExhaustiveEnumStructVariant {
    StructVariant { f: i32 },
}

// This pub enum's variant is no longer #[non_exhaustive], should trigger the lint.
#[non_exhaustive]
pub enum NonExhaustiveEnumStructVariant {
    StructVariant { f: i32 },
}

// This enum has a mix of variant types to test different variant kinds
pub enum MixedVariantKinds {
    // Unit variant to be no longer #[non_exhaustive]
    UnitVariant,
    // Tuple variant to be no longer #[non_exhaustive]
    TupleVariant(i32, i64),
    // Struct variant to be no longer #[non_exhaustive]
    StructVariant { field1: bool, field2: u32 },
}

// This enum's variant is already exhaustive and should not trigger the lint.
pub enum EnumWithAlreadyExhaustiveVariant {
    ExhaustiveVariant,
}

// This enum is hidden with #[doc(hidden)] and should not trigger the lint
#[doc(hidden)]
pub enum HiddenEnum {
    Variant,
}

// This enum has a hidden variant that should not trigger the lint
pub enum EnumWithHiddenVariant {
    #[doc(hidden)]
    HiddenVariantToBeExhaustive,
}

// This enum is private and should not trigger the lint
enum PrivateEnum {
    Variant,
}
