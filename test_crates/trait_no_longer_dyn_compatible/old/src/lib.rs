#![no_std]

// Object safe traits
pub trait RefTrait {
    fn by_ref(self: &Self) {}
}

mod sealed {
    pub trait Sealed {}
}

pub trait WillGainAssociatedConstWithDefault {}

pub trait WillGainAssociatedConstWithoutDefault {}

pub trait SealedWillGainAssociatedConstWithDefault: sealed::Sealed {}

pub trait SealedWillGainAssociatedConstWithoutDefault: sealed::Sealed {}

pub trait AlreadyDynIncompatible {
    fn make_me_non_dyn_compatible() -> Self;
}
