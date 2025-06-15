#![no_std]

pub struct Foo;

impl Foo {
    pub fn add_feature(&self) {}

    pub fn unaffected(&self) {}

    pub fn safe_add_feature(&self) {}

    #[target_feature(enable = "sse2")]
    pub unsafe fn already_requires_feature(&self) {}

    fn private_add_feature(&self) {}
}
