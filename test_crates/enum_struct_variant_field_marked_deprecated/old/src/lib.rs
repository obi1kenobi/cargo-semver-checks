#![no_std]

// Normal enum with struct variants whose fields will be marked as deprecated
pub enum NormalEnum {
    // Struct variant with fields that will be marked as deprecated
    StructVariant {
        // Will be marked as deprecated
        field_to_be_deprecated: i32,
        // Will be marked as deprecated with a message
        field_to_be_deprecated_with_message: i32,
        // Will remain normal
        normal_field: bool,
    },
    // Another variant type for comparison
    TupleVariant(
        // Will be marked as deprecated - not trigger
        i32,
        i32,
    ),
    // Unit variant
    UnitVariant,
}

// This enum will be marked as deprecated
// Its variant fields should not trigger the lint since the enum itself will be deprecated
pub enum EnumToBeDeprecated {
    StructVariant {
        field_in_enum_to_be_deprecated: i32,
        another_field: i32,
    },
}

// This enum is already deprecated
// Changes to its variant fields should not trigger the lint
#[deprecated]
pub enum AlreadyDeprecatedEnum {
    StructVariant {
        field_in_already_deprecated_enum: i32,
        another_field: i32,
    },
}

// This enum has a variant that will be marked as deprecated
// Its fields should not trigger the lint since the variant itself will be deprecated
pub enum EnumWithVariantToBeDeprecated {
    StructVariant {
        field_in_variant_to_be_deprecated: i32,
        another_field: i32,
    },
    AnotherVariant,
}

// This enum has a variant that is already deprecated
// Changes to its fields should not trigger the lint
pub enum EnumWithAlreadyDeprecatedVariant {
    #[deprecated]
    StructVariant {
        field_in_already_deprecated_variant: i32,
        another_field: i32,
    },
    AnotherVariant,
}

// This enum is private and should not trigger the lint
enum PrivateEnum {
    StructVariant {
        field_to_be_deprecated: i32,
        another_field: i32,
    },
}

// This enum has a hidden variant that should not trigger the lint
pub enum EnumWithHiddenVariant {
    NormalVariant,
    #[doc(hidden)]
    StructVariant {
        field_to_be_deprecated: i32,
        another_field: i32,
    },
}

// This enum is hidden and should not trigger the lint
#[doc(hidden)]
pub enum HiddenEnum {
    StructVariant {
        field_to_be_deprecated: i32,
        another_field: i32,
    },
}

// This enum has a struct variant with a field that is already deprecated
pub enum EnumWithAlreadyDeprecatedField {
    StructVariant {
        #[deprecated]
        already_deprecated_field: i32,
        field_to_be_deprecated: i32,
        normal_field: bool,
    },
}

// This enum has a struct variant with a hidden field that will be deprecated
pub enum EnumWithHiddenField {
    StructVariant {
        #[doc(hidden)]
        hidden_field_to_be_deprecated: i32,
        normal_field: bool,
    },
}
