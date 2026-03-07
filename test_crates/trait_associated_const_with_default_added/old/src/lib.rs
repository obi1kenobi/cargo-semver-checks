#![no_std]

mod sealed {
    pub(crate) trait Sealed {}
}

// These traits should be reported 
// These traits are non-sealed in BOTH old and new, and gain a new associated constant
// with a default value.

// Non-sealed trait will gain a constant with a default value.
pub trait WillGainConstWithDefault {
    fn make_me_non_dyn_compatible() -> Self;
}

// Non-sealed trait already has one constant, will gain another with a default value.
pub trait WillGainAnotherConstWithDefault {
    const ONE: bool;

    fn make_me_non_dyn_compatible() -> Self;
}

// Non-sealed trait will gain two new constants with default values (two results).
pub trait WillGainTwoConstsWithDefault {
    fn make_me_non_dyn_compatible() -> Self;
}

// Non-sealed trait will gain a doc-hidden constant with a default value.
// Doc-hidden items are still part of the API surface and should be reported.
pub trait WillGainDocHiddenConstWithDefault {
    fn make_me_non_dyn_compatible() -> Self;
}

// These traits should NOT be reported 

// Non-sealed trait will gain a constant WITHOUT a default value.
// This is a breaking change handled by the major lint `trait_associated_const_added`.
pub trait WillGainConstWithoutDefault {
    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed trait will gain a constant with a default value.
// Handled by the sealed lint `trait_associated_const_added_to_sealed_trait`.
pub trait SealedWillGainConstWithDefault: sealed::Sealed {
    fn make_me_non_dyn_compatible() -> Self;
}

// Public API sealed trait will gain a constant with a default value.
// Handled by the sealed lint `trait_associated_const_added_to_sealed_trait`.
pub trait PublicAPISealedWillGainConstWithDefault {
    #[doc(hidden)]
    type Hidden;

    fn make_me_non_dyn_compatible() -> Self;
}

// Non-sealed in old, will become sealed in new, and gain a const with default.
// Our lint must NOT fire because the trait is sealed in current.
pub trait WillBecomeSealedAndGainConstWithDefault {
    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed in old, will become non-sealed in new, and gain a const with default.
// Our lint must NOT fire because the trait was sealed in baseline.
pub trait SealedWillLoseSealAndGainConstWithDefault: sealed::Sealed {
    fn make_me_non_dyn_compatible() -> Self;
}

// Private trait will gain a constant with default. Not in the public API.
trait PrivateWillGainConstWithDefault {
    fn make_me_non_dyn_compatible() -> Self;
}

// Non-sealed trait that already has a constant with default. No new const added.
pub trait ConstWithDefaultAlreadyExists {
    const EXISTING: bool = true;

    fn make_me_non_dyn_compatible() -> Self;
}