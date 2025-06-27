#![no_std]

pub enum PublicEnum {
    // The basic case - should get reported
    TupleVariantWithFieldAdded(i32, u8),
    // If variant is non exhaustive, adding a field is not a breaking change
    #[non_exhaustive]
    NonExhaustiveTupleVariantWithFieldAdded(i32, u8),
    // If variant was non exhaustive, adding a field is not a breaking change
    #[non_exhaustive]
    TupleVariantWithFieldAddedBecomesExhaustive(i32, u8),
    // Variant becoming non exhaustive is a different kind of breaking change
    TupleVariantWithFieldAddedBecomesNonExhaustive(i32, u8),
    // Variant type change is a different kind of breaking change
    PlainVariantBecomesTuple,
    // Variant type change is a different kind of breaking change
    StructVariantBecomesTuple {},
}

// Changes in a private enum should not be reported
enum PrivateEnum {
    TupleVariantWithFieldAdded(i32, u8),
}

// Visibility change is a different kind of breaking change
enum BecomesPublicEnum {
    TupleVariantWithFieldAdded(i32, u8),
}

// Visibility change is a different kind of breaking change
pub enum BecomesPrivateEnum {
    TupleVariantWithFieldAdded(i32, u8),
}
