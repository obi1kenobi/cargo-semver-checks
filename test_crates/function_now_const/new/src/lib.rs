#![no_std]

pub const fn add(x: i64, y: i64) -> i64 {
    x + y
}

// New constant function added
// This should not trigger the lint
pub const fn new_const_fn(x: i64) -> i64 {
    x
}
