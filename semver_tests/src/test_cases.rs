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

#[cfg(not(feature = "struct_marked_non_exhaustive"))]
pub struct NonExternallyConstructibleStruct {
    pub foo: u64,

    // This private field means this struct cannot be constructed with a struct literal
    // from outside of this crate.
    bar: u64,
}

/// This should not be flagged as a semver violation of any kind.
/// This struct was not previously constructible externally using a struct literal
/// due to its private field, so the addition of non_exhaustive could not have broken that use.
#[cfg(feature = "struct_marked_non_exhaustive")]
#[non_exhaustive]
pub struct NonExternallyConstructibleStruct {
    pub foo: u64,

    // This private field means this struct cannot be constructed with a struct literal
    // from outside of this crate.
    bar: u64,
}
