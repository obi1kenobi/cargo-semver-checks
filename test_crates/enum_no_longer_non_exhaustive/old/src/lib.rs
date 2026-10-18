#![no_std]

#[non_exhaustive]
pub enum NonExhaustiveEnum {
    Variant,
}

// Not public API in the baseline, so this should not be reported.
#[doc(hidden)]
#[non_exhaustive]
pub enum PreviouslyHiddenEnum {
    Variant,
}

// Not public API in the current version, so this should not be reported.
#[non_exhaustive]
pub enum NowHiddenEnum {
    Variant,
}

// Only the visible reexport should be reported.
mod private {
    #[non_exhaustive]
    pub enum ReexportedEnum {
        Variant,
    }
}

#[doc(hidden)]
pub use private::ReexportedEnum as HiddenReexport;
pub use private::ReexportedEnum as PublicReexport;
