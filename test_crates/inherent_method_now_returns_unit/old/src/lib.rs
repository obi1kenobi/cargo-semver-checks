#![no_std]

pub struct PublicStruct;

impl PublicStruct {
    pub fn public_to_unit(&self) -> u8 {
        1
    }

    #[doc(hidden)]
    pub fn doc_hidden_to_unit(&self) -> u8 {
        1
    }

    fn private_to_unit(&self) -> u8 {
        1
    }

    // Async methods for additional coverage.

    pub async fn async_to_unit(&self) -> u8 {
        1
    }

    pub async fn async_now_sync(&self) -> u8 {
        1
    }

    pub fn now_async(&self) -> u8 {
        1
    }

    // Conversions involving `impl Future`.

    pub fn impl_future_now_async(&self) -> impl core::future::Future<Output = ()> {
        async {}
    }

    pub async fn async_now_impl_future(&self) {}

    pub fn impl_future_to_unit(&self) -> impl core::future::Future<Output = ()> {
        async {}
    }
}

mod private_module {
    pub struct HiddenStruct;

    impl HiddenStruct {
        pub fn to_unit(&self) -> u8 {
            1
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
