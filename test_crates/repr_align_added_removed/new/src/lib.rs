#![no_std]

// should trigger lint for becoming aligned
#[repr(align(8))]
pub struct StructBecomesAligned;

// should trigger lint for losing alignment
pub struct StructBecomesUnaligned;

// should only trigger lint for becoming private
#[repr(align(8))]
struct StructBecomesAlignedAndPrivate;

// should only trigger lint for becoming private
struct StructBecomesUnalignedAndPrivate;

// no lints expected
#[repr(align(8))]
pub struct StructStaysAligned;

// no lints expected
pub struct StructStaysUnaligned;

// no lints expected
#[repr(align(8))]
struct PrivateStructBecomesAligned;

// no lints expected
struct PrivateStructBecomesUnaligned;

// should trigger lint for becoming aligned
#[repr(align(8))]
pub enum EnumBecomesAligned {
    A,
}

// should trigger lint for losing alignment
pub enum EnumBecomesUnaligned {
    A,
}

// should only trigger lint for becoming private
#[repr(align(8))]
enum EnumBecomesAlignedAndPrivate {
    A,
}

// should only trigger lint for becoming private
enum EnumBecomesUnalignedAndPrivate {
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
#[repr(align(8))]
enum PrivateEnumBecomesAligned {
    A,
}

// no lints expected
enum PrivateEnumBecomesUnaligned {
    A,
}

// should trigger lint for becoming aligned
#[repr(align(8))]
pub union UnionBecomesAligned {
    a: u8,
}

// should trigger lint for losing alignment
pub union UnionBecomesUnaligned {
    a: u8,
}

// should only trigger lint for becoming private
#[repr(align(8))]
union UnionBecomesAlignedAndPrivate {
    a: u8,
}

// should only trigger lint for becoming private
union UnionBecomesUnalignedAndPrivate {
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
#[repr(align(8))]
union PrivateUnionBecomesAligned {
    a: u8,
}

// no lints expected
union PrivateUnionBecomesUnaligned {
    a: u8,
}
