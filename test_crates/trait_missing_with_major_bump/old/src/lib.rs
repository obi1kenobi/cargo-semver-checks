#![no_std]

pub trait RemovedTrait {}

pub unsafe trait RemovedUnsafeTrait {}

pub mod my_pub_mod {
    pub trait PubUseRemovedTrait {}
}

pub use my_pub_mod::PubUseRemovedTrait;

// This trait stops being unsafe. It is not removed and should not be reported.
pub unsafe trait TraitStopsBeingUnsafe {}

// This trait becomes unsafe. It is not removed and should not be reported.
pub trait TraitBecomesUnsafe {}

// This trait is private. Its removal is not breaking and should not be reported.
trait PrivateTrait {}
