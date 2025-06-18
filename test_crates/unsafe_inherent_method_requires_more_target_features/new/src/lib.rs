#![no_std]

pub struct Foo;

impl Foo {
    // Shouldn't lint, the orignal method was safe.
    #[target_feature(enable = "avx")]
    #[target_feature(enable = "avx2")]
    pub fn safe_method() {}

    // `avx2` is new, this should lint.
    #[target_feature(enable = "avx")]
    #[target_feature(enable = "avx2")]
    pub unsafe fn unsafe_method() {}

    // Private method, nothing to lint.
    #[target_feature(enable = "avx")]
    #[target_feature(enable = "avx2")]
    unsafe fn private_method() {}

    // The new feature is implied by the existing one, shouldn't lint.
    #[target_feature(enable = "avx2")]
    #[target_feature(enable = "avx")]
    pub unsafe fn implied_feature_method() {}

    // The new feature is already globally enabled on the target, shouldn't lint.
    #[target_feature(enable = "fma")]
    #[target_feature(enable = "sse2")]
    pub fn globally_enabled_method() {}

    // The method became safe *and* added a new feature. Should lint anyway.
    #[target_feature(enable = "avx")]
    #[target_feature(enable = "avx2")]
    pub fn becomes_safe() {}
}
