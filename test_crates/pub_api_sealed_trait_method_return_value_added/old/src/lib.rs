#![no_std]
#![allow(async_fn_in_trait)]

// Traits in a public API sealed module that should trigger the lint.
pub mod public_api_sealed_return_value_added {
    #[doc(hidden)]
    pub mod hidden {
        pub trait Sealed {}
    }

    pub trait ReturnsUnit: hidden::Sealed {
        fn method(&self);
    }

    #[doc(hidden)]
    pub trait DocHiddenReturnsUnit: hidden::Sealed {
        fn method(&self);
    }
}

pub mod public_api_sealed_async_return_value_added {
    #[doc(hidden)]
    pub mod hidden {
        pub trait Sealed {}
    }

    pub trait AsyncReturnsUnit: hidden::Sealed {
        async fn method(&self);
    }

    pub trait WasAsyncReturnsUnit: hidden::Sealed {
        async fn method(&self);
    }

    pub trait BecomeAsyncReturnsUnit: hidden::Sealed {
        fn method(&self);
    }
}

pub mod becomes_unconditionally_sealed {
    #[doc(hidden)]
    pub mod hidden {
        pub trait Sealed {}
    }

    pub trait ReturnsUnit: hidden::Sealed {
        fn method(&self);
    }
}

// Traits that are unconditionally sealed should be ignored by the lint.
pub mod unconditionally_sealed_return_value_added {
    mod hidden {
        pub trait Sealed {}
    }

    pub trait ReturnsUnit: hidden::Sealed {
        fn method(&self);
    }
}

// Private traits should be ignored by the lint.
mod private_return_value_added {
    pub mod hidden {
        pub trait Sealed {}
    }

    pub trait ReturnsUnit: hidden::Sealed {
        fn method(&self);
    }
}
