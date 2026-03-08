#![no_std]

mod sealed {
    pub(crate) trait Sealed {}
}

// These traits are sealed in BOTH old and new, and gained a new associated constant.

// Sealed trait gained a constant without a default value.
pub trait SealedWillGainConstWithoutDefault: sealed::Sealed {
    const BAR: bool;

    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed trait gained a constant with a default value.
// Still reported: the lint fires on any new const in a sealed trait, regardless of default.
pub trait SealedWillGainConstWithDefault: sealed::Sealed {
    const BAR: bool = true;

    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed trait already had one constant, gained another.
pub trait SealedWillGainAnotherConst: sealed::Sealed {
    const ONE: bool;
    const TWO: bool;

    fn make_me_non_dyn_compatible() -> Self;
}

// Public API sealed trait (sealed via doc-hidden associated type) gained a constant.
pub trait PublicAPISealedWillGainConst {
    #[doc(hidden)]
    type Hidden;

    const BAR: bool;

    fn make_me_non_dyn_compatible() -> Self;
}

// These traits should NOT be reported 

// Non-sealed trait gained a constant without default.
// This is a breaking change handled by the major lint `trait_associated_const_added`.
pub trait NonSealedWillGainConstWithoutDefault {
    const BAR: bool;

    fn make_me_non_dyn_compatible() -> Self;
}

// Non-sealed trait gained a constant with default.
// This is handled by `trait_associated_const_with_default_added`.
pub trait NonSealedWillGainConstWithDefault {
    const BAR: bool = true;

    fn make_me_non_dyn_compatible() -> Self;
}

// Was sealed in old, lost its seal, and gained a const.
// The const addition on a now-unsealed trait is breaking.
// Our lint does NOT fire because the trait is no longer sealed in current.
pub trait SealedWillLoseSealAndGainConst {
    const BAR: bool;

    fn make_me_non_dyn_compatible() -> Self;
}

// Was non-sealed in old, became sealed in new, and gained a const.
// The sealing itself is a breaking change (handled by `trait_newly_sealed`).
// Our lint does NOT fire because the trait was not sealed in baseline.
pub trait NonSealedWillGainSealAndConst: sealed::Sealed {
    const BAR: bool;

    fn make_me_non_dyn_compatible() -> Self;
}

// Private sealed trait gained a constant. Not in the public API, so not reported.
trait PrivateSealedWillGainConst: sealed::Sealed {
    const BAR: bool;

    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed trait that already had the constant. No new const, so not reported.
pub trait SealedConstAlreadyExists: sealed::Sealed {
    const EXISTING: bool;

    fn make_me_non_dyn_compatible() -> Self;
}