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

    #[cfg(not(feature = "inherent_method_missing"))]
    pub fn moved_trait_provided_method(&self) {}

    #[cfg(not(feature = "inherent_method_missing"))]
    pub fn moved_method(&self) {}
}

pub mod my_pub_mod {
    pub struct PubUseRemovedStruct;

    pub enum PubUseRemovedEnum {}

    pub fn pub_use_removed_fn() {}
}

#[cfg(not(feature = "struct_missing"))]
pub use my_pub_mod::PubUseRemovedStruct;

#[cfg(not(feature = "enum_missing"))]
pub use my_pub_mod::PubUseRemovedEnum;

#[cfg(not(feature = "function_missing"))]
pub use my_pub_mod::pub_use_removed_fn;

// Moving an inherent method to an implemented trait should not be a breaking change,
// both when the method is defined inside the trait and when it's implemented externally.
#[cfg(feature = "inherent_method_missing")]
pub trait Bar {
    fn moved_trait_provided_method(&self) {}

    fn moved_method(&self);
}

#[cfg(feature = "inherent_method_missing")]
impl Bar for Foo {
    fn moved_method(&self) {}
}
