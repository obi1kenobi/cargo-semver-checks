#![no_std]

use core::future::Future;

#[allow(async_fn_in_trait)]
pub trait Example {
    fn was_value() -> ();
    async fn async_was_value();
    fn impl_future_to_unit() -> ();
    async fn impl_future_now_async();
    fn async_now_impl_future() -> impl Future<Output = ()>;
    #[doc(hidden)]
    fn doc_hidden_was_value() -> ();
}

trait PrivateExample {
    fn private_was_value() -> ();
}
