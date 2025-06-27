#![no_std]

// should trigger lint for becoming packed
#[repr(packed)]
pub struct StructBecomesPacked;

// should trigger lint for losing packed
pub struct StructBecomesUnpacked;

// should only trigger lint for becoming private
#[repr(packed)]
struct StructBecomesPackedAndPrivate;

// should only trigger lint for becoming private
struct StructBecomesUnpackedAndPrivate;

// no lints expected
#[repr(packed)]
pub struct StructStaysPacked;

// no lints expected
pub struct StructStaysUnpacked;

// no lints expected
#[repr(packed)]
struct PrivateStructBecomesPacked;

// no lints expected
struct PrivateStructBecomesUnpacked;

// should trigger lint for becoming packed
#[repr(packed)]
pub union UnionBecomesPacked {
    field1: i32,
}

// should trigger lint for losing packed
pub union UnionBecomesUnpacked {
    field1: i32,
}

// should only trigger lint for becoming private
#[repr(packed)]
union UnionBecomesPackedAndPrivate {
    field1: i32,
}

// should only trigger lint for becoming private
union UnionBecomesUnpackedAndPrivate {
    field1: i32,
}

// no lints expected
#[repr(packed)]
pub union UnionStaysPacked {
    field1: i32,
}

// no lints expected
pub union UnionStaysUnpacked {
    field1: i32,
}

// no lints expected
#[repr(packed)]
union PrivateUnionBecomesPacked {
    field1: i32,
}

// no lints expected
union PrivateUnionBecomesUnpacked {
    field1: i32,
}
