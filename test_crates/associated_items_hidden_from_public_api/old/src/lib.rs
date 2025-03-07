#![no_std]

pub struct Example;

impl Example {
    #[doc(hidden)]
    pub fn directly_hidden_fn() {}

    #[doc(hidden)]
    pub const DIRECTLY_HIDDEN_CONST: i64 = 42;
}

#[doc(hidden)]
impl Example {
    pub fn impl_hidden_fn() {}

    pub const IMPL_HIDDEN_CONST: i64 = 42;
}
