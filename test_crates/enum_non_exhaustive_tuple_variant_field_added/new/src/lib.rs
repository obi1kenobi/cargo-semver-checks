#![no_std]

pub enum PublicEnum {
    #[non_exhaustive]
    TupleVariantWithFieldAdded(i32, i64, u8),
}

// Changes in a private enum should not be reported
enum PrivateEnum {
    #[non_exhaustive]
    TupleVariantWithFieldAdded(i32, i64, u8),
}
