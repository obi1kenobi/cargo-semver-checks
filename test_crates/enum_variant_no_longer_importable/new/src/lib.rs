#![no_std]

pub enum ReexportedVariant {
    RemovedVariant,
}

pub enum StillReexported {
    KeptVariant,
}

pub use StillReexported::KeptVariant;
