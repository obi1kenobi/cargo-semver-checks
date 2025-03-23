#![no_std]

pub const fn add(x: i64, y: i64) -> i64 {
    x + y
}

// This fn is private and should not trigger the lint.
const fn double(x: i32) -> i32 {
    x * 2
}

// This fn is hidden and should not trigger the lint.
#[doc(hidden)]
pub const fn hidden_fn(x: i32) -> i32 {
    x * 2
}

// New constant function added
// This should not trigger the lint
pub const fn new_const_fn(x: i64) -> i64 {
    x
}
