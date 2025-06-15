#![no_std]

pub struct A;

impl A {
    #[target_feature(enable = "avx")]
    pub unsafe fn foo() {}

    /// Not a breaking change on x86-64 since `sse2` is enabled by default.
    #[target_feature(enable = "sse2")]
    pub unsafe fn sse2_method() {}
    
    // Still a breaking change, even though it becomes safe.
    #[target_feature(enable = "avx")]
    pub fn becomes_safe() {}

    #[target_feature(enable = "avx")]
    pub fn safe() {}
}
