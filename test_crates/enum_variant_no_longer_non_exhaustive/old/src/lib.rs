#![no_std]

// This pub enum's variant is no longer #[non_exhaustive], should trigger the lint.
pub enum ExhaustiveEnumUnitVariant {
    #[non_exhaustive]
    UnitVariant,
}

// This pub enum's variant is no longer #[non_exhaustive], should trigger the lint.
#[non_exhaustive]
pub enum NonExhaustiveEnumUnitVariant {
    #[non_exhaustive]
    UnitVariant,
}

// This pub enum's variant is no longer #[non_exhaustive], should trigger the lint.
pub enum ExhaustiveEnumTupleVariant {
    #[non_exhaustive]
    TupleVariant(i32, i64),
}

// This pub enum's variant is no longer #[non_exhaustive], should trigger the lint.
#[non_exhaustive]
pub enum NonExhaustiveEnumTupleVariant {
    #[non_exhaustive]
    TupleVariant(i32, i64),
}

// This pub enum's variant is no longer #[non_exhaustive], should trigger the lint.
pub enum ExhaustiveEnumStructVariant {
    #[non_exhaustive]
    StructVariant { f: i32 },
}

// This pub enum's variant is no longer #[non_exhaustive], should trigger the lint.
#[non_exhaustive]
pub enum NonExhaustiveEnumStructVariant {
    #[non_exhaustive]
    StructVariant { f: i32 },
}

// This enum has a mix of variant types to test different variant kinds
pub enum MixedVariantKinds {
    // Unit variant to be no longer #[non_exhaustive]
    #[non_exhaustive]
    UnitVariant,
    // Tuple variant to be no longer #[non_exhaustive]
    #[non_exhaustive]
    TupleVariant(i32, i64),
    // Struct variant to be no longer #[non_exhaustive]
    #[non_exhaustive]
    StructVariant { field1: bool, field2: u32 },
}

// This enum's variant is already exhaustive and should not trigger the lint.
pub enum EnumWithAlreadyExhaustiveVariant {
    ExhaustiveVariant,
}


// This enum is hidden with #[doc(hidden)] and should not trigger the lint
#[doc(hidden)]
pub enum HiddenEnum {
    #[non_exhaustive]
    Variant,
}

// This enum has a hidden variant that should not trigger the lint
pub enum EnumWithHiddenVariant {
    #[non_exhaustive]
    #[doc(hidden)]
    HiddenVariantToBeExhaustive,
}

// This enum is private and should not trigger the lint
enum PrivateEnum {
    #[non_exhaustive]
    Variant,
}
