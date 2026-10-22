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

// Same-named methods in different inherent impls, unchanged between versions:
// the unit-returning `Cell` method must not be reported as a change of the `OnceCell` method.
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

// Same-named methods in different inherent impls where one of them did change:
// `Mixed<u8>::get` used to return a value, while `Mixed<u16>::get` always returned `()`.
pub struct Mixed<T>(T);

impl Mixed<u8> {
    pub fn get(&self) -> u8 {
        1
    }
}

impl Mixed<u16> {
    pub fn get(&self) {}
}
