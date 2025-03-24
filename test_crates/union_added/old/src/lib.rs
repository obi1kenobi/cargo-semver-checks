#![no_std]

// Existing Union should not trigger the lint.
pub union ExistingUnion {
    f1: i32,
}
