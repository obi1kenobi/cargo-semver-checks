pub trait TraitOne {}
pub trait TraitTwo {}

mod sealed {
    pub trait Sealed {}
}

pub trait Unchanged {}
pub trait UnchangedSealed: sealed::Sealed {}

pub trait WillGainOne: TraitOne {}
pub trait WillGainOneSealed: TraitOne + sealed::Sealed {}

pub trait WillGainAnotherOne: TraitOne + TraitTwo {}
pub trait WillGainAnotherOneSealed: TraitOne + TraitTwo + sealed::Sealed {}

pub trait WillGainStdOne: Sync {}
pub trait WillGainStdTwo: core::fmt::Debug {}
pub trait WillGainStdThree: PartialEq {
    fn make_me_non_object_safe() -> Self;
}

pub trait WillChangeStdToCore: core::fmt::Debug {}
