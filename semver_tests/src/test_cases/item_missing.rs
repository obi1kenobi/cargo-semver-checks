//! Testing: <https://doc.rust-lang.org/cargo/reference/semver.html#item-remove>

#[cfg(not(feature = "struct_missing"))]
pub struct WillBeRemovedStruct;

#[cfg(not(feature = "enum_missing"))]
pub enum WillBeRemovedEnum {}

#[cfg(not(feature = "function_missing"))]
pub fn will_be_removed_fn() {}
