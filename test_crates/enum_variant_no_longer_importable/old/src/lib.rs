#![no_std]

pub enum ReexportedVariant {
    /// Testing: <https://doc.rust-lang.org/cargo/reference/semver.html#item-remove>
    RemovedVariant,
}

pub use ReexportedVariant::RemovedVariant;

pub enum StillReexported {
    KeptVariant,
}

pub use StillReexported::KeptVariant;
