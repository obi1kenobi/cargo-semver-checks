//! Testing: <https://doc.rust-lang.org/cargo/reference/semver.html#item-remove>

#[cfg(not(feature = "struct_missing"))]
pub struct WillBeRemovedStruct;

#[cfg(not(feature = "enum_missing"))]
pub enum WillBeRemovedEnum {}

#[cfg(not(feature = "function_missing"))]
pub fn will_be_removed_fn() {}

// Test that associated function removal:
// - is caught and reported by `inherent_method_missing`
// - is not caught by `function_missing`.
pub struct Foo {}

impl Foo {
    #[cfg(not(feature = "function_missing"))]
    #[cfg(not(feature = "inherent_method_missing"))]
    pub fn will_be_removed_associated_fn() {}

    #[cfg(not(feature = "inherent_method_missing"))]
    pub fn will_be_removed_method(&self) {}
}
