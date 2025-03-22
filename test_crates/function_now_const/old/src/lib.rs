#![no_std]

pub fn add(x: i64, y: i64) -> i64 {
    x + y
}

// This fn is private and should not trigger the lint.
fn double(x: i32) -> i32 {
    x * 2
}

// This fn is hidden and should not trigger the lint.
#[doc(hidden)]
pub fn hidden_fn(x: i32) -> i32 {
    x * 2
}
