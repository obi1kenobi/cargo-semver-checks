#![no_std]

// true positive: struct with packed value changed
#[repr(packed(4))]
pub struct StructPacked1(i64);

// true positive: union with packed value changed
#[repr(packed(2))]
pub union UnionPacked2 {
    field1: i32,
}

// becomes private
#[repr(packed(1))]
pub struct StructPackedBecomesPrivate(i64);

// union becomes private
#[repr(packed(2))]
pub union UnionPackedBecomesPrivate {
    field1: i32,
}
