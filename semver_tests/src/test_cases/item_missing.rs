/// Testing: <https://doc.rust-lang.org/cargo/reference/semver.html#item-remove>
#[cfg(not(feature = "struct_missing"))]
pub struct WillBeRemovedStruct;

/// Testing: <https://doc.rust-lang.org/cargo/reference/semver.html#item-remove>
#[cfg(not(feature = "enum_missing"))]
pub enum WillBeRemovedEnum {}
