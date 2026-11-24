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

    // The old method was hidden. Exposing it with an AVX requirement does not
    // impose a new precondition on callers of the supported public API.
    #[target_feature(enable = "avx")]
    pub unsafe fn revealed() {}

    // Hidden, deprecated methods still belong to the supported public API.
    // Report the new AVX precondition on callers of A::legacy().
    #[doc(hidden)]
    #[deprecated]
    #[target_feature(enable = "avx")]
    pub unsafe fn legacy() {}
}
