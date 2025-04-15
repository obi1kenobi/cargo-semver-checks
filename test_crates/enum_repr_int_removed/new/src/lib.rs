#![no_std]

pub enum U8EnumToEnum {
    Bar,
    Baz,
}

pub enum I32EnumToEnum {
    Bar,
    Baz,
}

pub enum IsizeEnumToEnum {
    Bar,
    Baz,
}

pub enum UsizeEnumToEnum {
    Bar,
    Baz,
}

#[repr(C)]
pub enum U8CEnumToCEnum {
    Bar,
    Baz,
}

#[repr(C)]
pub enum SeparateU8CEnumToCEnum {
    Bar,
    Baz,
}

#[repr(C)]
pub enum CU8EnumToCEnum {
    Bar,
    Baz,
}

#[repr(C)]
pub enum SeparateCU8EnumToCEnum {
    Bar,
    Baz,
}


// The following enums have *changes* of repr(i*) and repr(u*),
// not removals of repr(i*) or repr(u*).
// They should not be reported by this rule, because they have their own rule.

#[repr(u16)]
pub enum U8ToU16Enum {
    Bar,
    Baz,
}

#[repr(i8)]
pub enum I32ToI8Enum {
    Bar,
    Baz,
}

#[repr(u32)]
pub enum I32ToU32Enum {
    Bar,
    Baz,
}

#[repr(C, u16)]
pub enum CU8ToCU16Enum {
    Bar,
    Baz,
}
