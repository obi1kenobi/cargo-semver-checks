#![no_std]

pub struct PublicStruct;

impl PublicStruct {
    pub fn public_to_unit(&self) {
        // previously returned a value
    }

    #[doc(hidden)]
    pub fn doc_hidden_to_unit(&self) {
        // previously returned a value
    }

    fn private_to_unit(&self) {
        // previously returned a value
    }

    // Async methods for additional coverage.

    pub async fn async_to_unit(&self) {
        // previously returned a value
    }

    pub fn async_now_sync(&self) {
        // previously async and returned a value
    }

    pub async fn now_async(&self) {
        // previously returned a value synchronously
    }

    // Conversions involving `impl Future`.

    pub async fn impl_future_now_async(&self) {
        // previously returned `impl Future`
    }

    pub fn async_now_impl_future(&self) -> impl core::future::Future<Output = ()> {
        async {}
    }

    pub fn impl_future_to_unit(&self) {
        // previously returned `impl Future`
    }
}

mod private_module {
    pub struct HiddenStruct;

    impl HiddenStruct {
        pub fn to_unit(&self) {
            // previously returned a value
        }
    }
}

// Same-named methods in different inherent impls are unchanged between versions,
// so they must not be paired with each other (rustdoc doesn't expose the impl's self type).
pub struct Wrapper<T>(T);

impl<T> Wrapper<core::cell::Cell<T>> {
    pub fn set(self, value: T) {
        self.0.set(value);
    }
}

impl<T> Wrapper<core::cell::OnceCell<T>> {
    pub fn set(self, value: T) -> Result<(), T> {
        self.0.set(value)
    }
}
