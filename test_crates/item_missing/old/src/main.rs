//! Testing: <https://doc.rust-lang.org/cargo/reference/semver.html#item-remove>

pub struct WillBeRemovedStruct;

pub enum WillBeRemovedEnum {}

pub fn will_be_removed_fn() {}

// Test that associated function removal:
// - is caught and reported by `inherent_method_missing`
// - is not caught by `function_missing`.
pub struct Foo {}

impl Foo {
    pub fn will_be_removed_associated_fn() {}

    pub fn will_be_removed_method(&self) {}

    pub fn moved_trait_provided_method(&self) {}

    pub fn moved_method(&self) {}
}

pub mod my_pub_mod {
    pub struct PubUseRemovedStruct;

    pub enum PubUseRemovedEnum {}

    pub fn pub_use_removed_fn() {}
}

pub use my_pub_mod::PubUseRemovedStruct;

pub use my_pub_mod::PubUseRemovedEnum;

pub use my_pub_mod::pub_use_removed_fn;
