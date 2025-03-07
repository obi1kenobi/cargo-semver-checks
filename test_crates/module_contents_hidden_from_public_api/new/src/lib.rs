#![no_std]

pub mod removals {}

#[doc(hidden)]
pub fn changed_fn_signature(x: i64, y: i64, z: i64) {
    assert_eq!(x + y, z);
}
