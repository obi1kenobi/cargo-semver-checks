#![no_std]

mod private {
    pub trait Sealed {}
}

#[doc(hidden)]
pub mod hidden {
    pub trait PublicSealed {}
}

pub trait ReturnValueAdded {
    fn method();
}

pub trait AsyncReturnValueAdded {
    async fn method();
}

pub trait WasAsyncAndValueAdded {
    async fn method();
}

pub trait BecomeAsyncAndValueAdded {
    fn method();
}

pub trait AlreadyUnconditionallySealed: private::Sealed {
    fn method();
}

pub trait AlreadyPublicApiSealed: hidden::PublicSealed {
    fn method();
}

pub trait NewlySealed {
    fn method();
}
