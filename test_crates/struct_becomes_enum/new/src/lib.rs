//! Changing a struct that isn't externally-constructible (e.g. due to private fields)
//! into an enum or union is not a breaking change by itself.
//!
//! If all of the struct's methods and trait impls continue to exist,
//! the change is not externally noticeable.
//!
//! The new enum is even allowed to become `#[non_exhaustive]`
//! without that being a breaking change.

#[non_exhaustive]
pub enum StructToEnum {
    Var(i64),
}

impl StructToEnum {
    pub fn new(x: i64) -> Self {
        Self::Var(x)
    }

    pub fn present_associated_fn() {}

    pub fn present_method(&self) {}
}


/// This struct is also not externally-constructible, due to `#[non_exhaustive]`.
/// Changing it to an enum is also not a breaking change.
///
/// If this struct had any public fields, the loss of those fields in the conversion to enum
/// would have been a breaking change. But as-is, this is not breaking.
#[non_exhaustive]
pub enum NonExhaustiveEmptyStructToEnum {
    Var,
}

impl NonExhaustiveEmptyStructToEnum {
    pub fn present_associated_fn() {}

    pub fn present_method(&self) {}
}
