#![no_std]

pub struct A;

impl A {
    pub unsafe fn foo() {}

    pub unsafe fn sse2_method() {}

    // Still a breaking change, even though it becomes safe.
    #[target_feature(enable = "avx")]
    pub unsafe fn becomes_safe() {}

    pub fn safe() {}
}
