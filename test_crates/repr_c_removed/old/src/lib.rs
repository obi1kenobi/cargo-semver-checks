#![no_std]

#[repr(C)]
pub enum CEnumToEnum {
    Bar,
}

#[repr(C, u8)]
pub enum CU8EnumToU8Enum {
    Bar,
    Baz,
}

#[repr(C)]
pub struct CStructToStruct {
    pub bar: usize,
}

#[repr(C, align(16))]
pub struct CAlign16StructToAlign16Struct {
    pub bar: usize,
}

#[repr(C)]
pub union CUnionToUnion {
    pub bar: usize,
}

#[repr(C, align(16))]
pub union CAlign16UnionToAlign16Union {
    pub bar: usize,
}
