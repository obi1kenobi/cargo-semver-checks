//! async fn and return position impl Future are equivalent.
//! Switching from one to the other should NOT generate any lints

use std::future::Future;

pub async fn switches_to_return_impl() {}

pub fn switches_to_async() -> impl Future {
    async {}
}

pub struct S;

impl S {
    pub async fn switches_to_return_impl() {}

    pub fn switches_to_async() -> impl Future {
        async {}
    }
}

#[allow(async_fn_in_trait)]
pub trait Trait {
    async fn switches_to_return_impl();
    fn switches_to_async() -> impl Future;
}
