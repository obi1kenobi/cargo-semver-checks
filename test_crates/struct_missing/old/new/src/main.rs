//! Testing: <https://doc.rust-lang.org/cargo/reference/semver.html#item-remove>

#[cfg(not(feature = "struct_missing"))]
pub struct WillBeRemovedStruct;

pub mod my_pub_mod {
    pub struct PubUseRemovedStruct;

    pub enum PubUseRemovedEnum {}

    pub fn pub_use_removed_fn() {}
}

#[cfg(not(feature = "struct_missing"))]
pub use my_pub_mod::PubUseRemovedStruct;

