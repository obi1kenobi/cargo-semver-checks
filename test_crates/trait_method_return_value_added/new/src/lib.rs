#![no_std]

mod private {
    pub trait Sealed {}
}

#[doc(hidden)]
pub mod hidden {
    pub trait PublicSealed {}
}

pub trait ReturnValueAdded {
    fn method() -> i32;
}

pub trait AsyncReturnValueAdded {
    async fn method() -> i32;
}

pub trait WasAsyncAndValueAdded {
    fn method() -> i32;
}

pub trait BecomeAsyncAndValueAdded {
    async fn method() -> i32;
}

pub trait AlreadyUnconditionallySealed: private::Sealed {
    fn method() -> i32;
}

pub trait AlreadyPublicApiSealed: hidden::PublicSealed {
    fn method() -> i32;
}

pub trait NewlySealed: hidden::PublicSealed {
    fn method() -> i32;
}
