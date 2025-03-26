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

#[repr(u8, C)]
pub enum CEnumToU8CEnum {
    Bar,
    Baz,
}

#[repr(u8)]
#[repr(C)]
pub enum CEnumToSeparateU8CEnum {
    Bar,
    Baz,
}

#[repr(C, u8)]
pub enum CEnumToCU8Enum {
    Bar,
    Baz,
}

#[repr(C)]
#[repr(u8)]
pub enum CEnumToSeparateCU8Enum {
    Bar,
    Baz,
}
