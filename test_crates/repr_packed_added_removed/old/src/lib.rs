// should trigger lint for becoming packed
pub struct StructBecomesPacked;

// should trigger lint for losing packed
#[repr(packed)]
pub struct StructBecomesUnpacked;

// should only trigger lint for becoming private
pub struct StructBecomesPackedAndPrivate;

// should only trigger lint for becoming private
#[repr(packed)]
pub struct StructBecomesUnpackedAndPrivate;

// no lints expected
#[repr(packed)]
pub struct StructStaysPacked;

// no lints expected
pub struct StructStaysUnpacked;

// no lints expected
struct PrivateStructBecomesPacked;

// no lints expected
#[repr(packed)]
struct PrivateStructBecomesUnpacked;

// should trigger lint for becoming packed
pub union UnionBecomesPacked {
    field1: i32,
}

// should trigger lint for losing packed
#[repr(packed)]
pub union UnionBecomesUnpacked {
    field1: i32,
}

// should only trigger lint for becoming private
pub union UnionBecomesPackedAndPrivate {
    field1: i32,
}

// should only trigger lint for becoming private
#[repr(packed)]
pub union UnionBecomesUnpackedAndPrivate {
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
union PrivateUnionBecomesPacked {
    field1: i32,
}

// no lints expected
#[repr(packed)]
union PrivateUnionBecomesUnpacked {
    field1: i32,
}
