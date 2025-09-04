#![no_std]

// true positive: struct with packed value changed
#[repr(packed(2))]
pub struct StructPacked1;

// true positive: union with packed value changed
#[repr(packed(4))]
pub union UnionPacked2 {
    field1: i32,
}

// packed value unchanged
#[repr(packed(1))]
pub struct StructPackedUnchanged;

// no repr(packed)
pub struct StructNoPacked;

// becomes private
#[repr(packed(2))]
struct StructPackedBecomesPrivate;

// union becomes private
#[repr(packed(4))]
union UnionPackedBecomesPrivate {
    field1: i32,
}

// union packed unchanged
#[repr(packed(2))]
pub union UnionPackedUnchanged {
    field1: i32,
}

// union without repr(packed)
pub union UnionNoPacked {
    field1: i32,
}
