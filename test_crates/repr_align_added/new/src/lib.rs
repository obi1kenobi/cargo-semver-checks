#![no_std]

// should trigger lint for gaining alignment
#[repr(align(16))]
pub struct StructBecomesAligned;

// should trigger lint for gaining alignment
#[repr(align(32))]
pub enum EnumBecomesAligned {
    A,
}

// should trigger lint for gaining alignment
#[repr(align(64))]
pub union UnionBecomesAligned {
    field1: i32,
}

// should only trigger lint for becoming private
#[repr(align(8))]
struct StructBecomesAlignedAndPrivate;

// should only trigger lint for becoming private
#[repr(align(8))]
enum EnumBecomesAlignedAndPrivate {
    A,
}

// should only trigger lint for becoming private
#[repr(align(8))]
union UnionBecomesAlignedAndPrivate {
    field1: i32,
}

// no lints expected
#[repr(align(8))]
pub struct StructStaysAligned;

// no lints expected
#[repr(align(8))]
pub enum EnumStaysAligned {
    A,
}

// no lints expected
#[repr(align(8))]
pub union UnionStaysAligned {
    field1: i32,
}

// no lints expected
pub struct StructStaysUnaligned;

// no lints expected
pub enum EnumStaysUnaligned {
    A,
}

// no lints expected
pub union UnionStaysUnaligned {
    field1: i32,
}

// no lints expected
#[repr(align(8))]
struct PrivateStructBecomesAligned;

// no lints expected
#[repr(align(8))]
enum PrivateEnumBecomesAligned {
    A,
}

// no lints expected
#[repr(align(8))]
union PrivateUnionBecomesAligned {
    field1: i32,
}
