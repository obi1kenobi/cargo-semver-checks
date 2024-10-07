//! Changing a struct that isn't externally-constructible (e.g. due to private fields)
//! into an enum or union is not a breaking change by itself.
//!
//! If all of the struct's methods and trait impls continue to exist,
//! the change is not externally noticeable.
//!
//! The new enum is even allowed to become `#[non_exhaustive]`
//! without that being a breaking change.


// The following structs cannot change to enums with a non-breaking change
// since their pub fields will become inaccessible.

pub struct PubFieldTuple(pub i64, bool);

pub struct PubFieldStruct {
    pub x: i64,
    _hidden: bool,
}

// Constructible fieldless structs cannot change to enums with a non-breaking change
// due to the inability to construct the struct using a struct literal.
//
// Normally, this case is subordinate to the breaking change regarding the inability
// to access pub fields. However, if the struct is exhaustive and has *no fields at all*
// (e.g. a unit or empty tuple struct), then it is still externally-constructible.

pub struct ConstructibleFieldlessUnit;

pub struct ConstructibleFieldlessTuple();

pub struct ConstructibleFieldlessStruct {}


// The following structs are not externally-constructible due to their private fields.
// They have no pub fields that might become inaccessible,
// so they can become enums with a non-breaking change.
//
// They are even allowed to (but not required to) become non-exhaustive without
// that being a breaking change, either.

pub struct TupleToEnum(i64);

impl TupleToEnum {
    pub fn new(x: i64) -> Self {
        Self(x)
    }

    pub fn present_associated_fn() {}

    pub fn present_method(&self) {}

    pub fn will_be_removed_associated_fn() {}

    pub fn will_be_removed_method(&self) {}
}


pub struct StructToEnum {
    x: i64
}

impl StructToEnum {
    pub fn new(x: i64) -> Self {
        Self { x }
    }

    pub fn present_associated_fn() {}

    pub fn present_method(&self) {}

    pub fn will_be_removed_associated_fn() {}

    pub fn will_be_removed_method(&self) {}
}


/// This struct is also not externally-constructible, due to `#[non_exhaustive]`.
/// Changing it to an enum is also not a breaking change.
///
/// If this struct had any public fields, the loss of those fields in the conversion to enum
/// would have been a breaking change. But as-is, this is not breaking.
#[non_exhaustive]
pub struct NonExhaustiveEmptyStructToEnum {}

impl NonExhaustiveEmptyStructToEnum {
    pub fn present_associated_fn() {}

    pub fn present_method(&self) {}
}

// The following structs are also not externally-constructible due to `#[non_exhaustive]`.

#[non_exhaustive]
pub struct NonExhaustiveFieldlessUnit;

#[non_exhaustive]
pub struct NonExhaustiveFieldlessTuple();
