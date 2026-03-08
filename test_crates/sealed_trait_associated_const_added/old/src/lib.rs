#![no_std]

mod sealed {
    pub(crate) trait Sealed {}
}

// These traits should be reported 
// These traits are sealed in BOTH old and new, and gain a new associated constant.

// Sealed trait will gain a constant without a default value.
pub trait SealedWillGainConstWithoutDefault: sealed::Sealed {
    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed trait will gain a constant with a default value.
// Still reported: the lint fires on any new const in a sealed trait, regardless of default.
pub trait SealedWillGainConstWithDefault: sealed::Sealed {
    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed trait already has one constant, will gain another.
pub trait SealedWillGainAnotherConst: sealed::Sealed {
    const ONE: bool;

    fn make_me_non_dyn_compatible() -> Self;
}

// Public API sealed trait (sealed via doc-hidden associated type) will gain a constant.
pub trait PublicAPISealedWillGainConst {
    #[doc(hidden)]
    type Hidden;

    fn make_me_non_dyn_compatible() -> Self;
}

// These traits should NOT be reported 

// Non-sealed trait will gain a constant without default.
// This is a breaking change handled by the major lint `trait_associated_const_added`.
pub trait NonSealedWillGainConstWithoutDefault {
    fn make_me_non_dyn_compatible() -> Self;
}

// Non-sealed trait will gain a constant with default.
// This is handled by `trait_associated_const_with_default_added`.
pub trait NonSealedWillGainConstWithDefault {
    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed in old, will become unsealed in new, and gain a const.
// The const addition on a now-unsealed trait is a breaking change, not a minor one.
// Our lint must NOT fire because the trait is no longer sealed in current.
pub trait SealedWillLoseSealAndGainConst: sealed::Sealed {
    fn make_me_non_dyn_compatible() -> Self;
}

// Non-sealed in old, will become sealed in new, and gain a const.
// The sealing itself is a breaking change (handled by `trait_newly_sealed`).
// Our lint must NOT fire because the trait was not sealed in baseline.
pub trait NonSealedWillGainSealAndConst {
    fn make_me_non_dyn_compatible() -> Self;
}

// Private sealed trait will gain a constant. Not in the public API, so not reported.
trait PrivateSealedWillGainConst: sealed::Sealed {
    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed trait that already has the constant. No new const is added, so not reported.
pub trait SealedConstAlreadyExists: sealed::Sealed {
    const EXISTING: bool;

    fn make_me_non_dyn_compatible() -> Self;
}