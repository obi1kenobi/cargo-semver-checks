#![no_std]

// These functions returned a value in the old version.
// Changing them to return unit should trigger the lint only for the public one.

pub fn PublicFnToUnit() -> u32 {
    0
}

#[doc(hidden)]
pub fn DocHiddenFnToUnit() -> u32 {
    0
}

fn PrivateFnToUnit() -> u32 {
    0
}

// Async functions for additional coverage.

pub async fn PublicAsyncFnToUnit() -> u32 {
    0
}

pub async fn PublicAsyncFnNowSync() -> u32 {
    0
}

pub fn PublicFnNowAsync() -> u32 {
    0
}
