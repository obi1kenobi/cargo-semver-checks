pub mod non_exhaustive;

/// Testing: <https://doc.rust-lang.org/cargo/reference/semver.html#item-remove>
#[cfg(not(feature = "struct_missing"))]
pub struct WillBeRemovedStruct;

pub struct FieldWillBeRemoved {
    pub foo: usize,

    /// Testing: <https://doc.rust-lang.org/cargo/reference/semver.html#item-remove>
    #[cfg(not(feature = "struct_pub_field_missing"))]
    pub bar: usize,
}

/// Testing: <https://doc.rust-lang.org/cargo/reference/semver.html#item-remove>
#[cfg(not(feature = "enum_missing"))]
pub enum WillBeRemovedEnum {}

pub enum VariantWillBeRemoved {
    Foo,

    /// Testing: <https://doc.rust-lang.org/cargo/reference/semver.html#item-remove>
    #[cfg(not(feature = "enum_variant_missing"))]
    Bar,
}

#[cfg(not(feature = "unit_struct_changed_kind"))]
pub struct UnitStructToPlain;

#[cfg(feature = "unit_struct_changed_kind")]
pub struct UnitStructToPlain {}

#[cfg(not(feature = "unit_struct_changed_kind"))]
#[non_exhaustive]
pub struct NonExhaustiveUnitStructToPlain;

#[cfg(feature = "unit_struct_changed_kind")]
#[non_exhaustive]
pub struct NonExhaustiveUnitStructToPlain {}

pub enum EnumWithNewVariant {
    OldVariant,

    #[cfg(feature = "enum_variant_added")]
    NewVariant,
}

/// This enum should not be reported by the `enum_variant_added` rule,
/// since it is non-exhaustive so adding new variants is not breaking.
#[non_exhaustive]
pub enum NonExhaustiveEnumWithNewVariant {
    OldVariant,

    #[cfg(feature = "enum_variant_added")]
    NewVariant,
}
