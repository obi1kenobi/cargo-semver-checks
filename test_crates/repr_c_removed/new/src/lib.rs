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

#[repr(align(16))]
pub struct CAlign16StructToAlign16Struct {
    pub bar: usize,
}

pub union CUnionToUnion {
    pub bar: usize,
}

#[repr(align(16))]
pub union CAlign16UnionToAlign16Union {
    pub bar: usize,
}
