#![no_std]

pub struct Foo;

impl Foo {
    // Should lint: safe method gains new feature
    #[target_feature(enable = "avx")]
    #[target_feature(enable = "avx2")]
    pub fn safe_method(&self) {}

    // Shouldn't lint: method is unsafe
    #[target_feature(enable = "avx")]
    #[target_feature(enable = "avx2")]
    pub unsafe fn unsafe_method(&self) {}

    // Became unsafe and added feature; another lint handles this
    #[target_feature(enable = "avx")]
    #[target_feature(enable = "avx2")]
    pub unsafe fn becomes_unsafe(&self) {}

    // New feature is implied by the existing one
    #[target_feature(enable = "avx2")]
    #[target_feature(enable = "avx")]
    pub fn implied_feature_method(&self) {}

    // New feature is already globally enabled
    #[target_feature(enable = "fma")]
    #[target_feature(enable = "sse2")]
    pub fn globally_enabled_method(&self) {}

    // Private method
    #[target_feature(enable = "avx")]
    #[target_feature(enable = "avx2")]
    fn private_method(&self) {}
}
