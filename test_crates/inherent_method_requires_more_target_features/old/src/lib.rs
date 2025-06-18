#![no_std]

pub struct Foo;

impl Foo {
    #[target_feature(enable = "avx")]
    pub fn safe_method() {}

    #[target_feature(enable = "avx")]
    pub unsafe fn unsafe_method() {}

    #[target_feature(enable = "avx")]
    fn private_method() {}

    #[target_feature(enable = "avx2")]
    pub fn implied_feature_method() {}

    #[target_feature(enable = "fma")]
    pub fn globally_enabled_method() {}
}
