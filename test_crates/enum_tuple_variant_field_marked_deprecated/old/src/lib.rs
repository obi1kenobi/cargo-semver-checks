#![no_std]

// This enum has tuple variants with fields that will be marked as deprecated
pub enum TupleVariantEnum {
    // This variant has fields that will be marked as deprecated
    TupleVariant(i32, u8, bool),

    // This variant will have some fields marked as deprecated
    AnotherTuple(u64, i16, char),

    // This will remain unchanged
    NormalTuple(f64, f32),

    // This variant will be entirely deprecated
    VariantToBeDeprecated(i32, u8, bool),

    // This variant is already deprecated
    #[deprecated]
    VariantAlreadyDeprecated(#[deprecated] i32),

    // Non-tuple variants for completeness
    UnitVariant,
    StructVariant {
        field1: bool,
        field2: u32,
    },
}

// This enum will be marked as deprecated in the new version
// Its tuple variant fields should not trigger the lint since the enum itself is deprecated
pub enum EnumToBeDeprecated {
    TupleVariant(i32, u8),
    UnitVariant,
}

// This enum is already deprecated
// Changes to its tuple variant fields should not trigger the lint
#[deprecated]
pub enum AlreadyDeprecatedEnum {
    TupleVariant(i32, u8),
    UnitVariant,
}

// This enum is private and should not trigger the lint
enum PrivateEnum {
    TupleVariant(i32, u8),
}

// This enum has a hidden variant that should not trigger the lint
pub enum EnumWithHiddenVariant {
    NormalTuple(i32, u8),
    #[doc(hidden)]
    HiddenTuple(i32, u8),
}

// This variant has already deprecated fields and should not trigger the lint
pub enum EnumWithAlreadyDeprecatedFields {
    NormalTuple(i32, u8),
    TupleWithDeprecatedField(#[deprecated] i32, u8),
    TupleWithDeprecatedFieldAndMessage(i32, #[deprecated = "Don't use this field"] u8),
}

// This enum is hidden with #[doc(hidden)] and should not trigger the lint
#[doc(hidden)]
pub enum HiddenEnum {
    TupleVariant(i32, u8),
}
