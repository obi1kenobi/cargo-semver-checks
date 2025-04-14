#![no_std]

#[repr(u8)]
pub enum U8EnumToEnum {
    Bar,
    Baz,
}

#[repr(i32)]
pub enum I32EnumToEnum {
    Bar,
    Baz,
}

#[repr(isize)]
pub enum IsizeEnumToEnum {
    Bar,
    Baz,
}

#[repr(usize)]
pub enum UsizeEnumToEnum {
    Bar,
    Baz,
}

#[repr(C, u8)]
pub enum CU8EnumToCEnum {
    Bar,
    Baz,
}


// The following enums have *changes* of repr(i*) and repr(u*),
// not removals of repr(i*) or repr(u*).
// They should not be reported by this rule, because they have their own rule.

#[repr(u8)]
pub enum U8ToU16Enum {
    Bar,
    Baz,
}

#[repr(i32)]
pub enum I32ToI8Enum {
    Bar,
    Baz,
}

#[repr(i32)]
pub enum I32ToU32Enum {
    Bar,
    Baz,
}

#[repr(C, u8)]
pub enum CU8ToCU16Enum {
    Bar,
    Baz,
}
