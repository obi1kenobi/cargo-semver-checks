#![no_std]

pub struct Foo;

impl Foo {
    pub fn associated_fn(x: i64, y: i64) -> i64 {
        x + y
    }

    pub fn method(&self, x: i64) -> i64 {
        x
    }

    pub unsafe fn associated_fn_removed() {}

    pub unsafe fn method_removed(&self) {}
}
