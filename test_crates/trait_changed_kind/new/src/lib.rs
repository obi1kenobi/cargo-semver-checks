#![no_std]

pub struct TraitToStruct;

pub enum TraitToEnum {
    Variant,
}

pub union TraitToUnion {
    value: u8,
}

#[allow(non_snake_case)]
pub fn TraitToFunction() {}

#[allow(non_snake_case)]
pub mod TraitToModule {}
