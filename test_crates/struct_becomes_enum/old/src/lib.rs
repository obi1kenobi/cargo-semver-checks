/// Changing a struct that isn't externally-constructible (e.g. due to private fields)
/// into an enum or union is not a breaking change by itself.
///
/// If all of the struct's methods and trait impls continue to exist,
/// the change is not externally noticeable.
///
/// The new enum is even allowed to become `#[non_exhaustive]`
/// without that being a breaking change.
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
