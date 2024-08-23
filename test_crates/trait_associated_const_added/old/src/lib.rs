mod sealed {
    pub(crate) trait Sealed {}
}

pub trait WillGainConstWithoutDefault {}

pub trait WillGainConstWithoutDefaultSealed: sealed::Sealed {}
pub trait WillGainConstWithoutDefaultAndSeal {}

pub trait WillGainConstWithDefault {}
pub trait WillGainConstWithDefaultSealed: sealed::Sealed {}

pub trait WillGainAnotherConstWithoutDefault {
    const ONE: bool;
}
pub trait WillGainAnotherConstWithoutDefaultSealed: sealed::Sealed {
    const ONE: bool;
}
