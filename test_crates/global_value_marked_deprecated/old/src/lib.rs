// These values will be marked deprecated
pub const VALUE_TO_DEPRECATED: i32 = 42;
pub static STATIC_TO_DEPRECATED: &str = "hello";
pub const fn CONST_FN_TO_DEPRECATED(x: i32) -> i32 {
    x + 1
}

// This const will become a static and be deprecated
pub const CONST_BECOMING_STATIC: u32 = 123;

// This static will become a const and be deprecated
pub static STATIC_BECOMING_CONST: i32 = 456;

pub const VALUE_TO_DEPRECATED_MESSAGE: bool = true;
pub static STATIC_TO_DEPRECATED_MESSAGE: char = 'x';
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

#[deprecated = "Old message"]
pub const VALUE_MESSAGE_CHANGES: &str = "old";

#[deprecated = "Old message"]
pub const fn CONST_FN_MESSAGE_CHANGES(x: u32) -> u32 {
    x
}

// Private values should not be reported
const PRIVATE_VALUE_TO_DEPRECATED: i32 = 0;
static PRIVATE_STATIC_TO_DEPRECATED: i32 = 1;
const fn PRIVATE_CONST_FN_TO_DEPRECATED(x: i32) -> i32 {
    x
}

// Hidden values should not be reported
#[doc(hidden)]
pub const HIDDEN_VALUE_TO_DEPRECATED: i32 = 2;

#[doc(hidden)]
pub static HIDDEN_STATIC_TO_DEPRECATED: i32 = 3;

#[doc(hidden)]
pub const fn HIDDEN_CONST_FN_TO_DEPRECATED(x: i32) -> i32 {
    x
}
