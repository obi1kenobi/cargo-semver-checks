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

// This enum already has a repr attribute, so adding an integer repr to it should not trigger this lint.
#[repr(C)]
pub enum CEnumToU8CEnum {
    Bar,
    Baz,
}

// This enum already has a repr attribute, so adding an integer repr to it should not trigger this lint.
#[repr(C)]
pub enum CEnumToSeparateU8CEnum {
    Bar,
    Baz,
}

// This enum already has a repr attribute, so adding an integer repr to it should not trigger this lint.
#[repr(C)]
pub enum CEnumToCU8Enum {
    Bar,
    Baz,
}

// This enum already has a repr attribute, so adding an integer repr to it should not trigger this lint.
#[repr(C)]
pub enum CEnumToSeparateCU8Enum {
    Bar,
    Baz,
}
