#![no_std]

#[doc(hidden)]
pub struct ExamplePlainStruct {
    pub x: i64,
}

#[doc(hidden)]
pub struct ExampleTupleStruct(i64);

#[doc(hidden)]
pub struct ExampleUnitStruct;

#[doc(hidden)]
pub enum ExampleEnum {
    First,
    Second,
}

#[doc(hidden)]
pub union ExampleUnion {
    pub signed: i64,
    pub unsigned: u64,
}
