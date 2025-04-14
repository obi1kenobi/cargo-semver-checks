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
pub enum CEnumToU8CEnum {
    Bar,
    Baz,
}

#[repr(C)]
pub enum CEnumToSeparateU8CEnum {
    Bar,
    Baz,
}

#[repr(C)]
pub enum CEnumToCU8Enum {
    Bar,
    Baz,
}

#[repr(C)]
pub enum CEnumToSeparateCU8Enum {
    Bar,
    Baz,
}
