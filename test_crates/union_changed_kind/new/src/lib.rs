#![no_std]

pub enum UnionToEnum {
    Variant,
}

pub trait UnionToTrait {}

#[allow(non_snake_case)]
pub fn UnionToFunction() {}

#[allow(non_snake_case)]
pub mod UnionToModule {}

#[doc(hidden)]
pub enum UnionToHiddenEnum {
    Variant,
}
