#![no_std]

// Normal enum with struct variants whose fields are now marked as deprecated
pub enum NormalEnum {
    // Struct variant with fields that are now deprecated
    StructVariant {
        // Now deprecated
        #[deprecated]
        field_to_be_deprecated: i32,
        // Now deprecated with a message
        #[deprecated = "Use normal_field instead"]
        field_to_be_deprecated_with_message: i32,
        // Still normal
        normal_field: bool,
    },
    // Another variant type for comparison
    TupleVariant(i32, i32),
    // Unit variant
    UnitVariant,
}

// This enum is now deprecated
// Its variant fields should not trigger the lint since the enum itself is deprecated
#[deprecated = "Use NormalEnum instead"]
pub enum EnumToBeDeprecated {
    StructVariant {
        // This should not trigger the lint since the enum is deprecated
        #[deprecated]
        field_in_enum_to_be_deprecated: i32,
        another_field: i32,
    },
}

// This enum was already deprecated
// Changes to its variant fields should not trigger the lint
#[deprecated]
pub enum AlreadyDeprecatedEnum {
    StructVariant {
        // This should not trigger the lint since the enum is already deprecated
        #[deprecated]
        field_in_already_deprecated_enum: i32,
        another_field: i32,
    },
}

// This enum has a variant that is now marked as deprecated
// Its fields should not trigger the lint since the variant itself is deprecated
pub enum EnumWithVariantToBeDeprecated {
    #[deprecated]
    StructVariant {
        // This should not trigger the lint since the variant is deprecated
        #[deprecated]
        field_in_variant_to_be_deprecated: i32,
        another_field: i32,
    },
    AnotherVariant,
}

// This enum has a variant that was already deprecated
// Changes to its fields should not trigger the lint
pub enum EnumWithAlreadyDeprecatedVariant {
    #[deprecated]
    StructVariant {
        // This should not trigger the lint since the variant was already deprecated
        #[deprecated]
        field_in_already_deprecated_variant: i32,
        another_field: i32,
    },
    AnotherVariant,
}

// This enum is private and should not trigger the lint
enum PrivateEnum {
    StructVariant {
        #[deprecated]
        field_to_be_deprecated: i32,
        another_field: i32,
    },
}

// This enum has a hidden variant that should not trigger the lint
pub enum EnumWithHiddenVariant {
    NormalVariant,
    #[doc(hidden)]
    StructVariant {
        #[deprecated]
        field_to_be_deprecated: i32,
        another_field: i32,
    },
}

// This enum is hidden and should not trigger the lint
#[doc(hidden)]
pub enum HiddenEnum {
    StructVariant {
        #[deprecated]
        field_to_be_deprecated: i32,
        another_field: i32,
    },
}

// This enum has a struct variant with a field that was already deprecated
pub enum EnumWithAlreadyDeprecatedField {
    StructVariant {
        #[deprecated]
        already_deprecated_field: i32,
        #[deprecated]
        field_to_be_deprecated: i32,
        normal_field: bool,
    },
}

// This enum has a struct variant with a hidden field that is now deprecated
pub enum EnumWithHiddenField {
    StructVariant {
        #[doc(hidden)]
        #[deprecated]
        hidden_field_to_be_deprecated: i32,
        normal_field: bool,
    },
}

// This is a new enum with deprecated fields in its struct variant
// It should not trigger the lint since it's a new enum
pub enum NewEnumWithDeprecatedFields {
    StructVariant {
        #[deprecated]
        deprecated_field: i32,
        normal_field: bool,
    },
}
