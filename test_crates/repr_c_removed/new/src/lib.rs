pub enum CEnumToEnum {
    Bar,
}

#[repr(u8)]
pub enum U8CEnumToU8Enum {
    Bar,
    Baz,
}

#[repr(u8)]
pub enum SeparateU8CEnumToU8Enum {
    Bar,
    Baz,
}

#[repr(u8)]
pub enum CU8EnumToU8Enum {
    Bar,
    Baz,
}

#[repr(u8)]
pub enum SeparateCU8EnumToU8Enum {
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

pub struct CStructToStruct {
    pub bar: usize,
}

#[repr(align(16))]
pub struct Align16CStructToAlign16Struct {
    pub bar: usize,
}

#[repr(align(16))]
pub struct SeparateAlign16CStructToAlign16Struct {
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
pub union Align16CUnionToAlign16Union {
    pub bar: usize,
}

#[repr(align(16))]
pub union SeparateAlign16CUnionToAlign16Union {
    pub bar: usize,
}

#[repr(align(16))]
pub union CAlign16UnionToAlign16Union {
    pub bar: usize,
}
