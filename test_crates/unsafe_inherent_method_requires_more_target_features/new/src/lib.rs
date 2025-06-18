#![no_std]

pub struct Foo;

impl Foo {
    #[target_feature(enable = "avx")]
    #[target_feature(enable = "avx2")]
    pub fn safe_method() {}

    #[target_feature(enable = "avx")]
    #[target_feature(enable = "avx2")]
    pub unsafe fn unsafe_method() {}

    #[target_feature(enable = "avx")]
    #[target_feature(enable = "avx2")]
    fn private_method() {}

    #[target_feature(enable = "avx2")]
    #[target_feature(enable = "avx")]
    pub fn implied_feature_method() {}

    #[target_feature(enable = "fma")]
    #[target_feature(enable = "sse2")]
    pub fn globally_enabled_method() {}
}
