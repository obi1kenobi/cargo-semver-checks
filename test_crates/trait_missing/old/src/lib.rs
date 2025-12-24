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

// This trait is not importable. Its removal is not breaking and should not be reported.
mod private_mod {
    pub trait NotImportableTrait {}
}

// This trait is replaced by a doc-hidden struct. It should still be reported.
pub trait DocHiddenReplacement {}
