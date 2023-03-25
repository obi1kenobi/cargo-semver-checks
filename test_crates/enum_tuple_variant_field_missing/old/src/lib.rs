pub enum PublicEnum {
    // The basic case - should get reported
    TupleVariantWithMissingField(i32, u8, String),
    // Variant type change is a different kind of breaking change
    TupleVariantBecomesPlain(i32),
    // Variant type change is a different kind of breaking change
    TupleVariantBecomesStruct(usize),
}

// Changes in a private enum should not be reported
enum PrivateEnum {
    TupleVariantWithMissingField(i32, u8, String),
}

// Visibility change is a different kind of breaking change
enum BecomesPublicEnum {
    TupleVariantWithMissingField(i32, u8, String),
}

// Visibility change is a different kind of breaking change
pub enum BecomesPrivateEnum {
    TupleVariantWithMissingField(i32, u8, String),
}
