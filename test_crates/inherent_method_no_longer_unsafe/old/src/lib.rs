#![no_std]

pub struct Foo;

impl Foo {
    // These two become safe in new -- should be caught.
    pub unsafe fn associated_fn(x: i64, y: i64) -> i64 {
        x + y
    }

    pub unsafe fn method(&self, x: i64) -> i64 {
        x
    }

    // Stays unsafe in new -- no match.
    pub unsafe fn stays_unsafe(&self) {}

    // Already safe -- no match.
    pub fn already_safe(&self) {}

    // Removed entirely in new -- different lint.
    pub unsafe fn will_be_removed(&self) {}
}

pub enum Bar {
    Var1,
    Var2,
}

impl Bar {
    // Becomes safe in new -- should be caught (works on enums too).
    pub unsafe fn unsafe_enum_associated_fn() {}
}

// Not pub, so changes here shouldn't be caught.
struct PrivateStruct;

impl PrivateStruct {
    pub unsafe fn becomes_safe(&self) {}
}
