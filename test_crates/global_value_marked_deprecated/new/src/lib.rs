#![no_std]

// These values are now deprecated and should be reported
#[deprecated]
pub const VALUE_TO_DEPRECATED: i32 = 42;

#[deprecated]
pub static STATIC_TO_DEPRECATED: &str = "hello";

// Changed from const to static and deprecated - should be reported
#[deprecated]
pub static CONST_BECOMING_STATIC: u32 = 123;

// Changed from static to const and deprecated - should be reported
#[deprecated]
pub const STATIC_BECOMING_CONST: i32 = 456;

#[deprecated = "This value is deprecated"]
pub const VALUE_TO_DEPRECATED_MESSAGE: bool = true;

#[deprecated = "This static is deprecated"]
pub static STATIC_TO_DEPRECATED_MESSAGE: char = 'x';

// These values should not trigger the lint
pub const VALUE_STAYS_NORMAL: u32 = 100;
pub static STATIC_STAYS_NORMAL: i64 = 200;

#[deprecated]
pub const VALUE_ALREADY_DEPRECATED: f64 = 3.14;

#[deprecated]
pub static STATIC_ALREADY_DEPRECATED: u8 = 255;

#[deprecated = "New message"]
pub const VALUE_MESSAGE_CHANGES: &str = "old";

// Private values should not be reported
#[deprecated]
const PRIVATE_VALUE_TO_DEPRECATED: i32 = 0;

#[deprecated]
static PRIVATE_STATIC_TO_DEPRECATED: i32 = 1;

// Hidden values should not be reported
#[doc(hidden)]
#[deprecated]
pub const HIDDEN_VALUE_TO_DEPRECATED: i32 = 2;

#[doc(hidden)]
#[deprecated]
pub static HIDDEN_STATIC_TO_DEPRECATED: i32 = 3;
