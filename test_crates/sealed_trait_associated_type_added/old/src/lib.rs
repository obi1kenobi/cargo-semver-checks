#![no_std]
#![feature(associated_type_defaults)]

mod sealed {
    pub(crate) trait Sealed {}
}

// These traits should be reported
// These traits are sealed in BOTH old and new, and gain a new associated type.

// Sealed trait will gain a type without a default value.
pub trait SealedWillGainTypeWithoutDefault: sealed::Sealed {
    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed trait will gain a type with a default value.
// Still reported: the lint fires on any new type in a sealed trait, regardless of default.
pub trait SealedWillGainTypeWithDefault: sealed::Sealed {
    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed trait already has one type, will gain another.
pub trait SealedWillGainAnotherType: sealed::Sealed {
    type One;

    fn make_me_non_dyn_compatible() -> Self;
}

// Public API sealed trait (sealed via doc-hidden associated type) will gain a type.
pub trait PublicAPISealedWillGainType {
    #[doc(hidden)]
    type Hidden;

    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed trait will gain two new types in one release (two results for same trait).
pub trait SealedWillGainTwoTypes: sealed::Sealed {
    fn make_me_non_dyn_compatible() -> Self;
}

// These traits should NOT be reported

// Non-sealed trait will gain a type without default.
// This is a breaking change handled by the major lint `trait_associated_type_added`.
pub trait NonSealedWillGainTypeWithoutDefault {
    fn make_me_non_dyn_compatible() -> Self;
}

// Non-sealed trait will gain a type with default.
// This is handled by `trait_associated_type_with_default_added`.
pub trait NonSealedWillGainTypeWithDefault {
    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed in old, will become unsealed in new, and gain a type.
// Our lint must NOT fire because the trait is no longer sealed in current.
pub trait SealedWillLoseSealAndGainType: sealed::Sealed {
    fn make_me_non_dyn_compatible() -> Self;
}

// Non-sealed in old, will become sealed in new, and gain a type.
// Our lint must NOT fire because the trait was not sealed in baseline.
pub trait NonSealedWillGainSealAndType {
    fn make_me_non_dyn_compatible() -> Self;
}

// Private sealed trait will gain a type. Not in the public API, so not reported.
trait PrivateSealedWillGainType: sealed::Sealed {
    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed trait that already has the type. No new type is added, so not reported.
pub trait SealedTypeAlreadyExists: sealed::Sealed {
    type Existing;

    fn make_me_non_dyn_compatible() -> Self;
}
