#![no_std]

use core::future::Future;

#[allow(async_fn_in_trait)]
pub trait Example {
    fn was_value() -> i32;

    async fn async_was_value() -> i32;

    fn impl_future_to_unit() -> impl Future<Output = ()>;

    fn impl_future_now_async() -> impl Future<Output = ()>;

    async fn async_now_impl_future();

    #[doc(hidden)]
    fn doc_hidden_was_value() -> i32;
}

trait PrivateExample {
    fn private_was_value() -> i32;
}
