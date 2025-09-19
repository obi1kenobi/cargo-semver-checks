#![no_std]

pub trait ReturnValueAdded {
    fn method();
}

mod private {
    pub trait Sealed {}
}
pub trait AlreadyUnconditionallySealed: private::Sealed {
    fn method();
}

#[doc(hidden)]
pub mod hidden {
    pub trait PublicSealed {}
    pub trait ToBeSealed {}
}
pub trait AlreadyPublicApiSealed: hidden::PublicSealed {
    fn method();
}

pub trait NewlySealed {
    fn method();
}
