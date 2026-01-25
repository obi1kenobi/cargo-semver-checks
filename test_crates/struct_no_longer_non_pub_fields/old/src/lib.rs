#![no_std]

pub struct MyStruct {
    pub pub_field: i32,
    private_field: i32,
}

pub struct AlwaysNonPubFieldStruct {
    pub pub_field: i32,
    private_field: i32,
}

pub struct AlwaysPubFieldStruct {
    pub pub_field: i32,
}

struct NonPubStruct {
    pub pub_field: i32,
    private_field: i32,
}

#[non_exhaustive]
pub struct NonExhaustiveStruct {
    pub pub_field: i32,
    private_field: i32,
}
