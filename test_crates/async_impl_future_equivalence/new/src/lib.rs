//! async fn and return position impl Future are equivalent.
//! Switching from one to the other should NOT generate any lints

use std::future::Future;

pub fn switches_to_return_impl() -> impl Future {
    async {}
}

pub async fn switches_to_async() {}

pub struct S;

impl S {
    pub fn switches_to_return_impl() -> impl Future {
        async {}
    }

    pub async fn switches_to_async() {}
}

// TODO: https://github.com/obi1kenobi/cargo-semver-checks/issues/624
// Uncomment once the project drops support for Rust < 1.75
//
// #[allow(async_fn_in_trait)]
// pub trait Trait {
//     fn switches_to_return_impl() -> impl Future;
//     async fn switches_to_async();
// }
