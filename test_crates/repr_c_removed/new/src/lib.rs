#![no_std]

pub enum CEnumToEnum {
    Bar,
}

#[repr(u8)]
pub enum CU8EnumToU8Enum {
    Bar,
    Baz,
}

pub struct CStructToStruct {
    pub bar: usize,
}

pub union CUnionToUnion {
    pub bar: usize,
}

#[repr(align(16))]
pub struct Align16CStructToAlign16Struct {
    pub bar: usize,
}

#[repr(align(16))]
pub union Align16CUnionToAlign16Union {
    pub bar: usize,
}
