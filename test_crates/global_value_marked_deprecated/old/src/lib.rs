#![no_std]

// These values will be marked deprecated
pub const VALUE_TO_DEPRECATED: i32 = 42;
pub static STATIC_TO_DEPRECATED: &str = "hello";

// This const will become a static and be deprecated
pub const CONST_BECOMING_STATIC: u32 = 123;

// This static will become a const and be deprecated
pub static STATIC_BECOMING_CONST: i32 = 456;

pub const VALUE_TO_DEPRECATED_MESSAGE: bool = true;
pub static STATIC_TO_DEPRECATED_MESSAGE: char = 'x';

// These values should not trigger the lint
pub const VALUE_STAYS_NORMAL: u32 = 100;
pub static STATIC_STAYS_NORMAL: i64 = 200;

#[deprecated]
pub const VALUE_ALREADY_DEPRECATED: f64 = 3.14;

#[deprecated]
pub static STATIC_ALREADY_DEPRECATED: u8 = 255;

#[deprecated = "Old message"]
pub const VALUE_MESSAGE_CHANGES: &str = "old";

// Private values should not be reported
const PRIVATE_VALUE_TO_DEPRECATED: i32 = 0;
static PRIVATE_STATIC_TO_DEPRECATED: i32 = 1;

// Hidden values should not be reported
#[doc(hidden)]
pub const HIDDEN_VALUE_TO_DEPRECATED: i32 = 2;

#[doc(hidden)]
pub static HIDDEN_STATIC_TO_DEPRECATED: i32 = 3;
