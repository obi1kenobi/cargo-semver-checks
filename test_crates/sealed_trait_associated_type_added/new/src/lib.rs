#![no_std]
#![feature(associated_type_defaults)]

mod sealed {
    pub(crate) trait Sealed {}
}

// These traits should be reported
// These traits are sealed in BOTH old and new, and gained a new associated type.

// Sealed trait gained a type without a default value.
pub trait SealedWillGainTypeWithoutDefault: sealed::Sealed {
    type Bar;

    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed trait gained a type with a default value.
// Still reported: the lint fires on any new type in a sealed trait, regardless of default.
pub trait SealedWillGainTypeWithDefault: sealed::Sealed {
    type Bar = bool;

    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed trait already had one type, gained another.
pub trait SealedWillGainAnotherType: sealed::Sealed {
    type One;
    type Two;

    fn make_me_non_dyn_compatible() -> Self;
}

// Public API sealed trait (sealed via doc-hidden associated type) gained a type.
pub trait PublicAPISealedWillGainType {
    #[doc(hidden)]
    type Hidden;

    type Bar;

    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed trait gained two new types in one release (two results for same trait).
pub trait SealedWillGainTwoTypes: sealed::Sealed {
    type Bar;
    type Baz;

    fn make_me_non_dyn_compatible() -> Self;
}

// These traits should NOT be reported

// Non-sealed trait gained a type without default.
// This is a breaking change handled by the major lint `trait_associated_type_added`.
pub trait NonSealedWillGainTypeWithoutDefault {
    type Bar;

    fn make_me_non_dyn_compatible() -> Self;
}

// Non-sealed trait gained a type with default.
// This is handled by `trait_associated_type_with_default_added`.
pub trait NonSealedWillGainTypeWithDefault {
    type Bar = bool;

    fn make_me_non_dyn_compatible() -> Self;
}

// Was sealed in old, lost its seal, and gained a type.
// Our lint does NOT fire because the trait is no longer sealed in current.
pub trait SealedWillLoseSealAndGainType {
    type Bar;

    fn make_me_non_dyn_compatible() -> Self;
}

// Was non-sealed in old, became sealed in new, and gained a type.
// Our lint does NOT fire because the trait was not sealed in baseline.
pub trait NonSealedWillGainSealAndType: sealed::Sealed {
    type Bar;

    fn make_me_non_dyn_compatible() -> Self;
}

// Private sealed trait gained a type. Not in the public API, so not reported.
trait PrivateSealedWillGainType: sealed::Sealed {
    type Bar;

    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed trait that already had the type. No new type, so not reported.
pub trait SealedTypeAlreadyExists: sealed::Sealed {
    type Existing;

    fn make_me_non_dyn_compatible() -> Self;
}
