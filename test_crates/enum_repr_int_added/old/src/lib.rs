#![no_std]

pub enum EnumToU8Enum {
    Bar,
    Baz,
}

pub enum EnumToI32Enum {
    Bar,
    Baz,
}

pub enum EnumToIsizeEnum {
    Bar,
    Baz,
}

pub enum EnumToUsizeEnum {
    Bar,
    Baz,
}

#[repr(C)]
pub enum CEnumToCU8Enum {
    Bar,
    Baz,
}
