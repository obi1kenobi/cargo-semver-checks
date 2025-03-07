#![no_std]

mod inner {
    pub trait Trait {}

    pub struct Struct {}

    pub enum Enum {
        First,
    }
}

pub use inner::*;
pub use inner::Enum::*;
