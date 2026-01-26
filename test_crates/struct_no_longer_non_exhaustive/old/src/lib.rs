#![no_std]

#[non_exhaustive]
pub struct MyStruct {
    pub field: i32,
}

#[non_exhaustive]
struct PrivateStruct {
    pub field: i32,
}

pub struct AlwaysExhaustiveStruct {
    pub field: i32,
}

#[non_exhaustive]
pub struct NeverExhaustiveStruct {
    pub field: i32,
}

#[non_exhaustive]
pub struct StructWithPrivateField {
    pub pub_field: i32,
    priv_field: i32,
}

#[non_exhaustive]
pub struct GainedPrivateField {
    pub pub_field: i32,
}
