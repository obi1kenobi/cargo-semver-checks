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

#[repr(C)]
pub union CUnionToUnion {
    pub bar: usize,
}

#[repr(align(16), C)]
pub struct Align16CStructToAlign16Struct {
    pub bar: usize,
}

#[repr(align(16), C)]
pub union Align16CUnionToAlign16Union {
    pub bar: usize,
}
