#![no_std]

// should trigger lint for gaining alignment
pub struct StructBecomesAligned;

// should trigger lint for gaining alignment
pub enum EnumBecomesAligned {
    A,
}

// should trigger lint for gaining alignment
pub union UnionBecomesAligned {
    field1: i32,
}

// should only trigger lint for becoming private
pub struct StructBecomesAlignedAndPrivate;

// should only trigger lint for becoming private
pub enum EnumBecomesAlignedAndPrivate {
    A,
}

// should only trigger lint for becoming private
pub union UnionBecomesAlignedAndPrivate {
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
struct PrivateStructBecomesAligned;

// no lints expected
enum PrivateEnumBecomesAligned {
    A,
}

// no lints expected
union PrivateUnionBecomesAligned {
    field1: i32,
}
