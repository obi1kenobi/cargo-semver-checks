mod sealed {
    pub trait Sealed {}
}

pub trait TraitOne {}
pub trait TraitTwo {}

pub trait Unchanged {}
pub trait UnchangedSealed: sealed::Sealed {}

pub trait WillGailOne {}
pub trait WillGailOneSealed: sealed::Sealed {}

pub trait WillGailAnotherOne: TraitOne {}
pub trait WillGailAnotherOneSealed: TraitOne + sealed::Sealed {}
