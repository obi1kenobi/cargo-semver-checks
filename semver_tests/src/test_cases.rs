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

/// Breaking change because of:
///
/// """
/// Non-exhaustive types cannot be constructed outside of the defining crate:
/// - Non-exhaustive variants (struct or enum variant) cannot be constructed with
///   a StructExpression (including with functional update syntax).
/// """
/// From: <https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute>
#[cfg(not(feature = "struct_marked_non_exhaustive"))]
pub struct ExternallyConstructibleStruct {
    pub foo: u64,
}

/// Breaking change because of:
///
/// """
/// Non-exhaustive types cannot be constructed outside of the defining crate:
/// - Non-exhaustive variants (struct or enum variant) cannot be constructed with
///   a StructExpression (including with functional update syntax).
/// """
/// From: <https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute>
#[cfg(feature = "struct_marked_non_exhaustive")]
#[non_exhaustive]
pub struct ExternallyConstructibleStruct {
    pub foo: u64,
}
