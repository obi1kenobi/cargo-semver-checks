#![no_std]

pub struct Foo;

impl Foo {
    #[target_feature(enable = "sse2")]
    pub unsafe fn add_feature(&self) {}

    pub fn unaffected(&self) {}

    #[target_feature(enable = "sse2")]
    pub fn safe_add_feature(&self) {}

    #[target_feature(enable = "sse2")]
    pub unsafe fn already_requires_feature(&self) {}

    #[target_feature(enable = "sse2")]
    unsafe fn private_add_feature(&self) {}
}
