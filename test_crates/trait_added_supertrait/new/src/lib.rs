mod sealed {
    pub trait Sealed {}
}

pub trait TraitOne {}
pub trait TraitTwo {}

pub trait Unchanged {}
pub trait UnchangedSealed: sealed::Sealed {}

pub trait WillGailOne: TraitOne {}
pub trait WillGailOneSealed: TraitOne + sealed::Sealed {}

pub trait WillGailAnotherOne: TraitOne + TraitTwo {}
pub trait WillGailAnotherOneSealed: TraitOne + TraitTwo + sealed::Sealed {}
