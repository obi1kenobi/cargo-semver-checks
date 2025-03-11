#![no_std]

// These enum variants are now marked as deprecated
pub enum NormalEnum {
    // Now deprecated
    #[deprecated]
    VariantToBeDeprecated,
    // Now deprecated with a message
    #[deprecated = "Use NormalVariant instead"]
    VariantToBeDeprecatedWithMessage,
    // Still normal
    NormalVariant,
}

// This enum is now deprecated
// Its variants should not trigger the lint since the enum itself is deprecated
#[deprecated = "Use NormalEnum instead"]
pub enum EnumToBeDeprecated {
    VariantInDeprecatedEnum,
    AnotherVariantInDeprecatedEnum,
}

// This enum was already deprecated
// Changes to its variants should not trigger the lint
#[deprecated]
pub enum AlreadyDeprecatedEnum {
    // This should not trigger the lint since the enum is already deprecated
    #[deprecated]
    VariantInAlreadyDeprecatedEnum,
    AnotherVariantInAlreadyDeprecatedEnum,
}

// This enum has a mix of variant types to test different variant kinds
pub enum MixedVariantKinds {
    // Unit variant now deprecated
    #[deprecated]
    UnitVariant,
    // Tuple variant now deprecated
    #[deprecated = "Use NormalTuple instead"]
    TupleVariant(i32, i64),
    // Struct variant now deprecated
    #[deprecated]
    StructVariant {
        field1: bool,
        field2: u32,
    },
    // These remain normal
    NormalUnit,
    NormalTuple(f64),
    NormalStruct {
        value: i32,
    },
}

// This enum is private and should not trigger the lint
enum PrivateEnum {
    #[deprecated]
    PrivateVariantToBeDeprecated,
    AnotherPrivateVariant,
}

// This enum has a hidden variant that should not trigger the lint
pub enum EnumWithHiddenVariant {
    NormalVariant,
    #[doc(hidden)]
    #[deprecated]
    HiddenVariantToBeDeprecated,
}

// This variant was already deprecated and should not trigger the lint
pub enum EnumWithAlreadyDeprecatedVariant {
    NormalVariant,
    #[deprecated]
    AlreadyDeprecatedVariant,
    #[deprecated = "New message"]
    VariantWithMessageToBeChanged,
}

// This is a new enum with deprecated variants
// It should not trigger the lint since it's a new enum
pub enum NewEnumWithDeprecatedVariants {
    NormalVariant,
    #[deprecated]
    DeprecatedVariant,
}

// This enum is hidden with #[doc(hidden)] and should not trigger the lint
#[doc(hidden)]
pub enum HiddenEnum {
    #[deprecated]
    VariantToBeDeprecated,
    AnotherVariant,
}
