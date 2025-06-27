#![no_std]

mod internal {
    pub struct OldName;

    pub enum OldEnum {
        Foo,
    }

    pub trait OldTrait {}

    pub fn old_fn() {}
}

pub use internal::{OldName, OldEnum, OldTrait, old_fn};
