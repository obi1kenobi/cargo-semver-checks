#![no_std]

pub struct A;

impl A {
    pub unsafe fn foo() {}

    pub unsafe fn sse2_method() {}

    pub fn safe() {}
}
