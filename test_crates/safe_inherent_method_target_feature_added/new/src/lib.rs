#![no_std]

pub struct Foo;

impl Foo {
    // Don't trigger the lint here, let the `safe -> unsafe` lint handle this.
    #[target_feature(enable = "bmi1")]
    pub unsafe fn become_unsafe(&self) {}

    pub fn unaffected(&self) {}

    // This has a false-negative in rustdoc versions prior to v46,
    // because they incorrectly mark the function `unsafe` because of `#[target_feature]`.
    #[target_feature(enable = "sse2")]
    pub fn safe_add_feature(&self) {}

    #[target_feature(enable = "bmi1")]
    pub fn already_requires_feature(&self) {}

    #[target_feature(enable = "sse2")]
    fn private_add_feature(&self) {}
}
