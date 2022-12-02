//! Testing: <https://doc.rust-lang.org/cargo/reference/semver.html#item-remove>

// Test that associated function removal:
// - is caught and reported by `inherent_method_missing`
// - is not caught by `function_missing`.
pub struct Foo {}

impl Foo {}

pub mod my_pub_mod {
    pub struct PubUseRemovedStruct;

    pub enum PubUseRemovedEnum {}

    pub fn pub_use_removed_fn() {}
}

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
