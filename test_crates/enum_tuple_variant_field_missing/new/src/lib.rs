pub enum PublicEnum {
    // The basic case - should get reported
    TupleVariantWithMissingField(i32, String),
    // Variant type change is a different kind of breaking change
    TupleVariantBecomesPlain,
    // Variant type change is a different kind of breaking change
    TupleVariantBecomesStruct {},
}

// Changes in a private enum should not be reported
enum PrivateEnum {
    TupleVariantWithMissingField(i32, String),
}

// Visibility change is a different kind of breaking change
pub enum BecomesPublicEnum {
    TupleVariantWithMissingField(i32, String),
}

// Visibility change is a different kind of breaking change
enum BecomesPrivateEnum {
    TupleVariantWithMissingField(i32, String),
}
