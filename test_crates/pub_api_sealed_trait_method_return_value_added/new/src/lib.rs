#![no_std]
#![allow(async_fn_in_trait)]

// Trait remains public API sealed but now returns a non-unit value.
pub mod public_api_sealed_return_value_added {
    #[doc(hidden)]
    pub mod hidden {
        pub trait Sealed {}
    }

    pub trait ReturnsUnit: hidden::Sealed {
        fn method(&self) -> u8;
    }

    #[doc(hidden)]
    pub trait DocHiddenReturnsUnit: hidden::Sealed {
        fn method(&self) -> u8;
    }
}

pub mod public_api_sealed_async_return_value_added {
    #[doc(hidden)]
    pub mod hidden {
        pub trait Sealed {}
    }

    pub trait AsyncReturnsUnit: hidden::Sealed {
        async fn method(&self) -> u8;
    }

    pub trait WasAsyncReturnsUnit: hidden::Sealed {
        fn method(&self) -> u8;
    }

    pub trait BecomeAsyncReturnsUnit: hidden::Sealed {
        async fn method(&self) -> u8;
    }
}

pub mod becomes_unconditionally_sealed {
    mod hidden {
        pub trait Sealed {}
    }

    pub trait ReturnsUnit: hidden::Sealed {
        fn method(&self) -> u8;
    }
}

// Unconditionally sealed traits should still be ignored.
pub mod unconditionally_sealed_return_value_added {
    mod hidden {
        pub trait Sealed {}
    }

    pub trait ReturnsUnit: hidden::Sealed {
        fn method(&self) -> u8;
    }
}

// Private traits should be ignored by the lint.
mod private_return_value_added {
    pub mod hidden {
        pub trait Sealed {}
    }

    pub trait ReturnsUnit: hidden::Sealed {
        fn method(&self) -> u8;
    }
}
