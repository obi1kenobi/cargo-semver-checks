#![no_std]

mod sealed {
    pub(crate) trait Sealed {}
}

// These traits should be reported 
// These traits are non-sealed in BOTH old and new, and gained a new associated constant
// with a default value.

// Non-sealed trait gained a constant with a default value.
pub trait WillGainConstWithDefault {
    const BAR: bool = true;

    fn make_me_non_dyn_compatible() -> Self;
}

// Non-sealed trait already had one constant, gained another with a default value.
pub trait WillGainAnotherConstWithDefault {
    const ONE: bool;
    const TWO: bool = false;

    fn make_me_non_dyn_compatible() -> Self;
}

// Non-sealed trait gained two new constants with default values (two results).
pub trait WillGainTwoConstsWithDefault {
    const FOO: bool = true;
    const BAR: bool = false;

    fn make_me_non_dyn_compatible() -> Self;
}

// Non-sealed trait gained a doc-hidden constant with a default value.
// Doc-hidden items are still part of the API surface and should be reported.
pub trait WillGainDocHiddenConstWithDefault {
    #[doc(hidden)]
    const BAR: bool = true;

    fn make_me_non_dyn_compatible() -> Self;
}

// These traits should NOT be reported 

// Non-sealed trait gained a constant WITHOUT a default value.
// This is a breaking change handled by the major lint `trait_associated_const_added`.
pub trait WillGainConstWithoutDefault {
    const BAR: bool;

    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed trait gained a constant with a default value.
// Handled by the sealed lint `trait_associated_const_added_to_sealed_trait`.
pub trait SealedWillGainConstWithDefault: sealed::Sealed {
    const BAR: bool = true;

    fn make_me_non_dyn_compatible() -> Self;
}

// Public API sealed trait gained a constant with a default value.
// Handled by the sealed lint `trait_associated_const_added_to_sealed_trait`.
pub trait PublicAPISealedWillGainConstWithDefault {
    #[doc(hidden)]
    type Hidden;

    const BAR: bool = true;

    fn make_me_non_dyn_compatible() -> Self;
}

// Was non-sealed, became sealed, and gained a const with default.
// Our lint does NOT fire because the trait is sealed in current.
pub trait WillBecomeSealedAndGainConstWithDefault: sealed::Sealed {
    const BAR: bool = true;

    fn make_me_non_dyn_compatible() -> Self;
}

// Was sealed, lost its seal, and gained a const with default.
// Our lint does NOT fire because the trait was sealed in baseline.
pub trait SealedWillLoseSealAndGainConstWithDefault {
    const BAR: bool = true;

    fn make_me_non_dyn_compatible() -> Self;
}

// Private trait gained a constant with default. Not in the public API.
trait PrivateWillGainConstWithDefault {
    const BAR: bool = true;

    fn make_me_non_dyn_compatible() -> Self;
}

// Non-sealed trait that already had a constant with default. No new const added.
pub trait ConstWithDefaultAlreadyExists {
    const EXISTING: bool = true;

    fn make_me_non_dyn_compatible() -> Self;
}