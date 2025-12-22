#![no_std]

pub struct EnumToStruct;

pub union EnumToUnion {
    value: u8,
}

pub trait EnumToTrait {}

#[allow(non_snake_case)]
pub fn EnumToFunction() {}

pub struct EnumEmpty;

pub struct EnumEmptyNonExhaustive;

#[allow(non_snake_case)]
pub mod EnumToModule {}
