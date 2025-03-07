#![no_std]

pub struct Foo;

impl Foo {
    pub unsafe fn associated_fn(x: i64, y: i64) -> i64 {
        x + y
    }

    pub unsafe fn method(&self, x: i64) -> i64 {
        x
    }

    pub unsafe fn new_unsafe_associated_fn() {}

    pub unsafe fn new_unsafe_method(&self) {}
}
