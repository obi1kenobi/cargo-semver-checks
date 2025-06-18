#![no_std]

pub struct Foo;

impl Foo {
    pub fn become_unsafe(&self) {}

    pub fn unaffected(&self) {}

    pub fn safe_add_feature(&self) {}

    #[target_feature(enable = "bmi1")]
    pub fn already_requires_feature(&self) {}

    fn private_add_feature(&self) {}
}
