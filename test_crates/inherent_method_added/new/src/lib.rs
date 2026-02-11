#![no_std]

pub struct MyStruct;

impl MyStruct {
    /// An existing public method — should NOT trigger.
    pub fn existing_method(&self) -> i32 {
        42
    }

    /// A newly added public method — SHOULD trigger.
    pub fn new_method(&self) -> bool {
        true
    }

    /// A newly added private method — should NOT trigger.
    fn private_new_method(&self) {}

    /// A newly added public static method (no receiver) — SHOULD trigger.
    pub fn new_static_method() -> &'static str {
        "hello"
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

    /// A newly added public method on an enum — SHOULD trigger.
    pub fn new_enum_method(&self) -> i32 {
        0
    }
}

// Non-public struct — should NOT trigger even if methods are added.
struct PrivateStruct;

impl PrivateStruct {
    pub fn private_struct_method(&self) {}
    pub fn another_private_struct_method(&self) {}
}
