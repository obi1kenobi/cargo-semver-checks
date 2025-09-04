#![no_std]

// should trigger lint for becoming aligned
pub struct StructBecomesAligned;

// should trigger lint for losing alignment
#[repr(align(8))]
pub struct StructBecomesUnaligned;

// should only trigger lint for becoming private
pub struct StructBecomesAlignedAndPrivate;

// should only trigger lint for becoming private
#[repr(align(8))]
pub struct StructBecomesUnalignedAndPrivate;

// no lints expected
#[repr(align(8))]
pub struct StructStaysAligned;

// no lints expected
pub struct StructStaysUnaligned;

// no lints expected
struct PrivateStructBecomesAligned;

// no lints expected
#[repr(align(8))]
struct PrivateStructBecomesUnaligned;

// should trigger lint for becoming aligned
pub enum EnumBecomesAligned {
    A,
}

// should trigger lint for losing alignment
#[repr(align(8))]
pub enum EnumBecomesUnaligned {
    A,
}

// should only trigger lint for becoming private
pub enum EnumBecomesAlignedAndPrivate {
    A,
}

// should only trigger lint for becoming private
#[repr(align(8))]
pub enum EnumBecomesUnalignedAndPrivate {
    A,
}

// no lints expected
#[repr(align(8))]
pub enum EnumStaysAligned {
    A,
}

// no lints expected
pub enum EnumStaysUnaligned {
    A,
}

// no lints expected
enum PrivateEnumBecomesAligned {
    A,
}

// no lints expected
#[repr(align(8))]
enum PrivateEnumBecomesUnaligned {
    A,
}

// should trigger lint for becoming aligned
pub union UnionBecomesAligned {
    a: u8,
}

// should trigger lint for losing alignment
#[repr(align(8))]
pub union UnionBecomesUnaligned {
    a: u8,
}

// should only trigger lint for becoming private
pub union UnionBecomesAlignedAndPrivate {
    a: u8,
}

// should only trigger lint for becoming private
#[repr(align(8))]
pub union UnionBecomesUnalignedAndPrivate {
    a: u8,
}

// no lints expected
#[repr(align(8))]
pub union UnionStaysAligned {
    a: u8,
}

// no lints expected
pub union UnionStaysUnaligned {
    a: u8,
}

// no lints expected
union PrivateUnionBecomesAligned {
    a: u8,
}

// no lints expected
#[repr(align(8))]
union PrivateUnionBecomesUnaligned {
    a: u8,
}
