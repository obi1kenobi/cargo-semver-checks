#![no_std]

// These enum variants will be marked as deprecated in the new version
pub enum NormalEnum {
    // Will be marked as deprecated
    VariantToBeDeprecated,
    // Will be marked as deprecated with a message
    VariantToBeDeprecatedWithMessage,
    // Will remain normal
    NormalVariant,
}

// This enum will be marked as deprecated in the new version
// Its variants should not trigger the lint since the enum itself is deprecated
pub enum EnumToBeDeprecated {
    VariantInDeprecatedEnum,
    AnotherVariantInDeprecatedEnum,
}

// This enum is already deprecated
// Changes to its variants should not trigger the lint
#[deprecated]
pub enum AlreadyDeprecatedEnum {
    VariantInAlreadyDeprecatedEnum,
    AnotherVariantInAlreadyDeprecatedEnum,
}

// This enum has a mix of variant types to test different variant kinds
pub enum MixedVariantKinds {
    // Unit variant to be deprecated
    UnitVariant,
    // Tuple variant to be deprecated
    TupleVariant(i32, i64),
    // Struct variant to be deprecated
    StructVariant { field1: bool, field2: u32 },
    // These will remain normal
    NormalUnit,
    NormalTuple(f64),
    NormalStruct { value: i32 },
}

// This enum is private and should not trigger the lint
enum PrivateEnum {
    PrivateVariantToBeDeprecated,
    AnotherPrivateVariant,
}

// This enum has a hidden variant that should not trigger the lint
pub enum EnumWithHiddenVariant {
    NormalVariant,
    #[doc(hidden)]
    HiddenVariantToBeDeprecated,
}

// This variant is already deprecated and should not trigger the lint
pub enum EnumWithAlreadyDeprecatedVariant {
    NormalVariant,
    #[deprecated]
    AlreadyDeprecatedVariant,
    #[deprecated = "Old message"]
    VariantWithMessageToBeChanged,
}

// This enum is hidden with #[doc(hidden)] and should not trigger the lint
#[doc(hidden)]
pub enum HiddenEnum {
    VariantToBeDeprecated,
    AnotherVariant,
}
