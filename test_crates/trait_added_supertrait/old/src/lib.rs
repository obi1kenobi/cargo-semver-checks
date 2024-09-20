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
    fn make_me_non_object_safe() -> Self;
}

pub trait WillChangeStdToCore: std::fmt::Debug {}
