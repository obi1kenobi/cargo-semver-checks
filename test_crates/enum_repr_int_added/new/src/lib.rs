#![no_std]

#[repr(u8)]
pub enum EnumToU8Enum {
    Bar,
    Baz,
}

#[repr(i32)]
pub enum EnumToI32Enum {
    Bar,
    Baz,
}

#[repr(isize)]
pub enum EnumToIsizeEnum {
    Bar,
    Baz,
}

#[repr(usize)]
pub enum EnumToUsizeEnum {
    Bar,
    Baz,
}

#[repr(C, u8)]
pub enum CEnumToCU8Enum {
    Bar,
    Baz,
}
