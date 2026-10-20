#![no_std]

mod sealed {
    pub(crate) trait Sealed {}
}

// Sealed and not dyn-compatible. The new constants, with and without a default value,
// should be reported. The new #[doc(hidden)] constant should not.
pub trait SealedNotDynCompatible: sealed::Sealed {
    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed from the public API's point of view because of the #[doc(hidden)] associated type.
// The new constant should be reported.
pub trait PublicApiSealedNotDynCompatible {
    #[doc(hidden)]
    type Hidden;

    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed in the baseline, so the new constant should be reported
// even though the trait stops being sealed.
pub trait BecomesUnsealed: sealed::Sealed {
    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed but dyn-compatible: the new constant makes it no longer dyn-compatible,
// which `trait_no_longer_dyn_compatible` reports, so this lint should not report it.
pub trait SealedDynCompatible: sealed::Sealed {
    fn method(&self);
}

// Not sealed: `trait_associated_const_added` reports the new constant without a default value,
// so this lint should not report either new constant.
pub trait NotSealed {
    fn make_me_non_dyn_compatible() -> Self;
}

// Not sealed in the baseline, so this lint should not report the new constant
// even though the trait becomes sealed.
pub trait BecomesSealed {
    fn make_me_non_dyn_compatible() -> Self;
}

// Not public API, so this lint should not report it.
#[doc(hidden)]
pub trait HiddenTrait: sealed::Sealed {
    fn make_me_non_dyn_compatible() -> Self;
}

mod private {
    // Not importable from outside the crate, so this lint should not report it.
    pub trait PrivateTrait: super::sealed::Sealed {
        fn make_me_non_dyn_compatible() -> Self;
    }
}
