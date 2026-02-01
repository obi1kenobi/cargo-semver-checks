#![no_std]

// This struct's method becomes const and should be reported.
pub struct Foo;

impl Foo {
    pub const fn becomes_const(&self) -> i32 {
        42
    }

    pub const fn associated_fn_becomes_const() -> i32 {
        100
    }
}

// This enum's method becomes const and should be reported.
pub enum Bar {
    Variant,
}

impl Bar {
    pub const fn enum_method_becomes_const(&self) -> i32 {
        10
    }
}

// This union's method becomes const and should be reported.
pub union Baz {
    pub field: i32,
}

impl Baz {
    pub const fn union_method_becomes_const(&self) -> i32 {
        unsafe { self.field }
    }
}

// This public struct's method is private and should NOT be reported.
pub struct PrivateMethod;

impl PrivateMethod {
    const fn private_becomes_const(&self) -> i32 {
        42
    }
}

// This struct is private and should NOT be reported.
struct PrivateStruct;

impl PrivateStruct {
    pub const fn private_struct_method_becomes_const(&self) -> i32 {
        42
    }
}

// This struct is doc(hidden) and should NOT be reported.
#[doc(hidden)]
pub struct HiddenStruct;

impl HiddenStruct {
    pub const fn hidden_struct_method_becomes_const(&self) -> i32 {
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

// This struct and its const method were added in the new version.
// It should NOT be reported by this lint to avoid duplicate lints.
// Made pub(crate) to avoid interfering with other lint tests.
pub(crate) struct NewStruct;

impl NewStruct {
    pub const fn new_const_method() -> i32 {
        200
    }
}
