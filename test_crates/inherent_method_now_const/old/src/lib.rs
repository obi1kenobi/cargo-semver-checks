#![no_std]

// This struct's method becomes const and should be reported.
pub struct Foo;

impl Foo {
    pub fn becomes_const(&self) -> i32 {
        42
    }

    pub fn associated_fn_becomes_const() -> i32 {
        100
    }
}

// This enum's method becomes const and should be reported.
pub enum Bar {
    Variant,
}

impl Bar {
    pub fn enum_method_becomes_const(&self) -> i32 {
        10
    }
}

// This union's method becomes const and should be reported.
pub union Baz {
    pub field: i32,
}

impl Baz {
    pub fn union_method_becomes_const(&self) -> i32 {
        unsafe { self.field }
    }
}

// This public struct's method is private and should NOT be reported.
pub struct PrivateMethod;

impl PrivateMethod {
    fn private_becomes_const(&self) -> i32 {
        42
    }
}

// This struct is private and should NOT be reported.
struct PrivateStruct;

impl PrivateStruct {
    pub fn private_struct_method_becomes_const(&self) -> i32 {
        42
    }
}

// This struct is doc(hidden) and should NOT be reported.
#[doc(hidden)]
pub struct HiddenStruct;

impl HiddenStruct {
    pub fn hidden_struct_method_becomes_const(&self) -> i32 {
        42
    }
}

// This public struct's method was already const and should NOT be reported.
pub struct AlreadyConst;

impl AlreadyConst {
    pub const fn already_was_const(&self) -> i32 {
        42
    }
}
