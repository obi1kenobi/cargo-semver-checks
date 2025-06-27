//! Do not add `#![no_std]` here! This crate contains a test
//! that ensures that switching between referring to the std and core exports of the same trait
//! is a no-op and does not trigger lints.

pub trait TraitOne {}
pub trait TraitTwo {}

mod sealed {
    pub trait Sealed {}
}

pub trait Unchanged {}
pub trait UnchangedSealed: sealed::Sealed {}

pub trait WillGainOne {}
pub trait WillGainOneSealed: sealed::Sealed {}

pub trait WillGainAnotherOne: TraitOne {}
pub trait WillGainAnotherOneSealed: TraitOne + sealed::Sealed {}

pub trait WillGainStdOne {}
pub trait WillGainStdTwo {}
pub trait WillGainStdThree {
    fn make_me_non_dyn_compatible() -> Self;
}

pub trait WillChangeStdToCore: std::fmt::Debug {}

pub trait PublicAPISealed {
    #[doc(hidden)]
    type Hidden;
}
