#![no_std]

// These functions now return unit.

pub fn PublicFnToUnit() {}

#[doc(hidden)]
pub fn DocHiddenFnToUnit() {}

fn PrivateFnToUnit() {}

// Async functions for additional coverage.

pub async fn PublicAsyncFnToUnit() {}

pub fn PublicAsyncFnNowSync() {}

pub async fn PublicFnNowAsync() {}

// Conversions involving `impl Future`.

pub async fn PublicImplFutureNowAsync() {}

pub fn PublicAsyncFnNowImplFuture() -> impl core::future::Future<Output = ()> {
    async {}
}

pub fn PublicImplFutureToUnit() {}
