#![no_std]

pub struct MyStruct;

impl MyStruct {
    /// An existing public method — should NOT trigger.
    pub fn existing_method(&self) -> i32 {
        42
    }
}

pub enum MyEnum {
    A,
    B,
}

impl MyEnum {
    /// An existing public method on an enum — should NOT trigger.
    pub fn existing_enum_method(&self) -> bool {
        true
    }
}

// Non-public struct — should NOT trigger even if methods are added.
struct PrivateStruct;

impl PrivateStruct {
    pub fn private_struct_method(&self) {}
}
