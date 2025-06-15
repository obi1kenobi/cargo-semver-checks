#![no_std]

pub struct A;

impl A {
    #[target_feature(enable = "avx")]
    pub unsafe fn foo() {}

    /// Not a breaking change on x86-64 since `sse2` is enabled by default.
    #[target_feature(enable = "sse2")]
    pub unsafe fn sse2_method() {}

    #[target_feature(enable = "avx")]
    pub fn safe() {}
}
