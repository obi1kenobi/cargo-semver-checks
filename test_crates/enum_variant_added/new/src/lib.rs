pub enum EnumWithNewVariant {
    OldVariant,

    NewVariant,
}

/// This enum should not be reported by the `enum_variant_added` rule,
/// since it is non-exhaustive so adding new variants is not breaking.
#[non_exhaustive]
pub enum NonExhaustiveEnumWithNewVariant {
    OldVariant,

    NewVariant,
}
