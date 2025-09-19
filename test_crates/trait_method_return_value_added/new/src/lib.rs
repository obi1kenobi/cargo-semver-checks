#![no_std]

pub trait ReturnValueAdded {
    fn method() -> i32;
}

mod private {
    pub trait Sealed {}
}
pub trait AlreadyUnconditionallySealed: private::Sealed {
    fn method() -> i32;
}

#[doc(hidden)]
pub mod hidden {
    pub trait PublicSealed {}
    pub trait ToBeSealed {}
}
pub trait AlreadyPublicApiSealed: hidden::PublicSealed {
    fn method() -> i32;
}

pub trait NewlySealed: hidden::ToBeSealed {
    fn method() -> i32;
}
