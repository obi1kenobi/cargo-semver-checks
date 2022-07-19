//! Adding `non_exhaustive` to a struct or enum variant is breaking because those items
//! cannot be constructed outside of their defining crate:
//!
//! """
/// Non-exhaustive types cannot be constructed outside of the defining crate:
/// - Non-exhaustive variants (struct or enum variant) cannot be constructed with
///   a StructExpression (including with functional update syntax).
/// """
/// From: <https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute>

#[cfg(not(feature = "struct_marked_non_exhaustive"))]
pub struct UnitStruct;

#[cfg(feature = "struct_marked_non_exhaustive")]
#[non_exhaustive]
pub struct UnitStruct;

#[cfg(not(feature = "struct_marked_non_exhaustive"))]
pub struct TupleStruct(pub u64);

#[cfg(feature = "struct_marked_non_exhaustive")]
#[non_exhaustive]
pub struct TupleStruct(pub u64);

#[cfg(not(feature = "struct_marked_non_exhaustive"))]
pub struct ExternallyConstructibleStruct {
    pub foo: u64,
}

#[cfg(feature = "struct_marked_non_exhaustive")]
#[non_exhaustive]
pub struct ExternallyConstructibleStruct {
    pub foo: u64,
}

// The private field within means this struct cannot be constructed
// outside this crate, so #[non_exhaustive] won't change anything here.
#[cfg(not(feature = "struct_marked_non_exhaustive"))]
pub struct NonExternallyConstructibleTupleStruct(u64);

#[cfg(feature = "struct_marked_non_exhaustive")]
#[non_exhaustive]
pub struct NonExternallyConstructibleTupleStruct(u64);

#[cfg(not(feature = "struct_marked_non_exhaustive"))]
pub struct NonExternallyConstructibleStruct {
    pub foo: u64,

    // This private field means this struct cannot be constructed with a struct literal
    // from outside of this crate.
    bar: u64,
}

#[cfg(feature = "struct_marked_non_exhaustive")]
#[non_exhaustive]
pub struct NonExternallyConstructibleStruct {
    pub foo: u64,

    // This private field means this struct cannot be constructed with a struct literal
    // from outside of this crate.
    bar: u64,
}
