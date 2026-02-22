#![no_std]

pub struct Foo;

impl Foo {
    // Was unsafe, now safe.
    pub fn associated_fn(x: i64, y: i64) -> i64 {
        x + y
    }

    // Was unsafe, now safe.
    pub fn method(&self, x: i64) -> i64 {
        x
    }

    // Still unsafe.
    pub unsafe fn stays_unsafe(&self) {}

    // Was already safe.
    pub fn already_safe(&self) {}

    // will_be_removed is gone.
}

pub enum Bar {
    Var1,
    Var2,
}

impl Bar {
    // Was unsafe, now safe.
    pub fn unsafe_enum_associated_fn() {}
}

struct PrivateStruct;

impl PrivateStruct {
    pub fn becomes_safe(&self) {}
}
