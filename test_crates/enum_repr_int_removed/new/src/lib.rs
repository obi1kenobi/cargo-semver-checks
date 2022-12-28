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

#[repr(u8, C)]
pub enum SeparateU8CToU8CEnum {
    Bar,
    Baz,
}

#[repr(u8)]
#[repr(C)]
pub enum U8CToSeparateU8CEnum {
    Bar,
    Baz,
}

#[repr(C, u8)]
pub enum SeparateCU8ToCU8Enum {
    Bar,
    Baz,
}

#[repr(C)]
#[repr(u8)]
pub enum CU8ToSeparateCU8Enum {
    Bar,
    Baz,
}

#[repr(C, u8)]
pub enum SeparateU8CToCU8Enum {
    Bar,
    Baz,
}

#[repr(u8, C)]
pub enum SeparateCU8ToU8CEnum {
    Bar,
    Baz,
}

#[repr(u8, C)]
pub enum CU8ToU8CEnum {
    Bar,
    Baz,
}

#[repr(C, u8)]
pub enum U8CToCU8Enum {
    Bar,
    Baz,
}

#[repr(u8)]
#[repr(C)]
pub enum SeparateCU8ToSeparateU8CEnum {
    Bar,
    Baz,
}
