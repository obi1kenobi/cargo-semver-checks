#![no_std]

pub struct A;

impl A {
    pub unsafe fn foo() {}

    pub unsafe fn sse2_method() {}

    pub unsafe fn becomes_safe() {}

    pub fn safe() {}

    #[doc(hidden)]
    pub unsafe fn revealed() {}

    #[doc(hidden)]
    #[deprecated]
    pub unsafe fn legacy() {}
}
