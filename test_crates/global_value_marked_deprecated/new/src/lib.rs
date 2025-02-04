// These values are now deprecated and should be reported
#[deprecated]
pub const VALUE_TO_DEPRECATED: i32 = 42;

#[deprecated]
pub static STATIC_TO_DEPRECATED: &str = "hello";

#[deprecated]
pub const fn CONST_FN_TO_DEPRECATED(x: i32) -> i32 {
    x + 1
}

#[deprecated = "This value is deprecated"]
pub const VALUE_TO_DEPRECATED_MESSAGE: bool = true;

#[deprecated = "This static is deprecated"]
pub static STATIC_TO_DEPRECATED_MESSAGE: char = 'x';

#[deprecated = "This const fn is deprecated"]
pub const fn CONST_FN_TO_DEPRECATED_MESSAGE(s: &str) -> usize {
    s.len()
}

// These values should not trigger the lint
pub const VALUE_STAYS_NORMAL: u32 = 100;
pub static STATIC_STAYS_NORMAL: i64 = 200;
pub const fn CONST_FN_STAYS_NORMAL(x: f64) -> f64 {
    x * 2.0
}

#[deprecated]
pub const VALUE_ALREADY_DEPRECATED: f64 = 3.14;

#[deprecated]
pub static STATIC_ALREADY_DEPRECATED: u8 = 255;

#[deprecated]
pub const fn CONST_FN_ALREADY_DEPRECATED(x: char) -> bool {
    x.is_ascii()
}

#[deprecated = "New message"]
pub const VALUE_MESSAGE_CHANGES: &str = "old";

#[deprecated = "New message"]
pub const fn CONST_FN_MESSAGE_CHANGES(x: u32) -> u32 {
    x
}

// Private values should not be reported
#[deprecated]
const PRIVATE_VALUE_TO_DEPRECATED: i32 = 0;

#[deprecated]
static PRIVATE_STATIC_TO_DEPRECATED: i32 = 1;

#[deprecated]
const fn PRIVATE_CONST_FN_TO_DEPRECATED(x: i32) -> i32 {
    x
}

// Hidden values should not be reported
#[doc(hidden)]
#[deprecated]
pub const HIDDEN_VALUE_TO_DEPRECATED: i32 = 2;

#[doc(hidden)]
#[deprecated]
pub static HIDDEN_STATIC_TO_DEPRECATED: i32 = 3;

#[doc(hidden)]
#[deprecated]
pub const fn HIDDEN_CONST_FN_TO_DEPRECATED(x: i32) -> i32 {
    x
}
