pub struct ExamplePlainStruct {
    pub x: i64,
}

pub struct ExampleTupleStruct(i64);

pub struct ExampleUnitStruct;

pub enum ExampleEnum {
    First,
    Second,
}

pub union ExampleUnion {
    pub signed: i64,
    pub unsigned: u64,
}

/// This module gets removed completely, and this is not a breaking change
/// because it was never public API in the first place.
///
/// Since the module itself is also `#[doc(hidden)]`, it shouldn't get reported
/// by the `module_missing` lint either.
#[doc(hidden)]
pub mod to_be_removed {
    pub struct RemovedPlainStruct {
        pub x: i64,
    }

    pub struct RemovedTupleStruct(i64);

    pub struct RemovedUnitStruct;

    pub enum RemovedEnum {
        First,
        Second,
    }

    pub union RemovedUnion {
        pub signed: i64,
        pub unsigned: u64,
    }
}
