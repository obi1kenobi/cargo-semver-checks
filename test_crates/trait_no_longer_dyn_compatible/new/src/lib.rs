#![no_std]

// Non-object safe traits
pub trait RefTrait {
    fn by_ref(self) -> Self;
}

mod sealed {
    pub trait Sealed {}
}

pub trait WillGainAssociatedConstWithDefault {
    const N: i64 = 0;
}

pub trait WillGainAssociatedConstWithoutDefault {
    const N: i64;
}

pub trait SealedWillGainAssociatedConstWithDefault: sealed::Sealed {
    const N: i64 = 0;
}

pub trait SealedWillGainAssociatedConstWithoutDefault: sealed::Sealed {
    const N: i64;
}

pub trait AlreadyDynIncompatible {
    const N: i64 = 0;

    fn make_me_non_dyn_compatible() -> Self;
}
