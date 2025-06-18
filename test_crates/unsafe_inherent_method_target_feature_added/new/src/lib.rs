#![no_std]

pub struct A;

impl A {
    // Breaking change, this should lint.
    #[target_feature(enable = "avx")]
    pub unsafe fn foo() {}

    /// Not a breaking change on x86-64 since `sse2` is enabled by default.
    #[target_feature(enable = "sse2")]
    pub unsafe fn sse2_method() {}

    // Still a breaking change for this lint, even though it becomes safe.
    #[target_feature(enable = "avx")]
    pub fn becomes_safe() {}

    // Breaking change, but for a different lint.
    #[target_feature(enable = "avx")]
    pub fn safe() {}
}
