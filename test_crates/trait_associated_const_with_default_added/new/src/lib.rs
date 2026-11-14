#![no_std]

mod sealed {
    pub(crate) trait Sealed {}
}

// Not sealed and not dyn-compatible. The new constant with a default value should be reported.
// The new constant without a default value and the new #[doc(hidden)] constant should not.
pub trait NotDynCompatible {
    const ADDED_WITH_DEFAULT: i64 = 0;
    const ADDED_WITHOUT_DEFAULT: i64;
    #[doc(hidden)]
    const ADDED_HIDDEN: i64 = 0;

    fn make_me_non_dyn_compatible() -> Self;
}

// Not dyn-compatible because of its existing constant. The new constant should be reported,
// but the existing constant gaining a default value should not.
pub trait HasConst {
    const EXISTING: i64 = 0;
    const ADDED: i64 = Self::EXISTING + 1;
}

// Not sealed in the baseline, so the new constant should be reported
// even though the trait becomes sealed.
pub trait BecomesSealed: sealed::Sealed {
    const ADDED: i64 = 0;

    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed: `sealed_trait_associated_const_added` reports the new constant,
// so this lint should not report it.
pub trait SealedTrait: sealed::Sealed {
    const ADDED: i64 = 0;

    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed in the baseline, so this lint should not report the new constant
// even though the trait stops being sealed.
pub trait BecomesUnsealed {
    const ADDED: i64 = 0;

    fn make_me_non_dyn_compatible() -> Self;
}

// Dyn-compatible, but the new constant makes it no longer dyn-compatible.
// The new constant should be reported, and `trait_no_longer_dyn_compatible` also reports the trait.
pub trait DynCompatible {
    const ADDED: i64 = 0;

    fn method(&self);
}

// Not public API, so this lint should not report it.
#[doc(hidden)]
pub trait HiddenTrait {
    const ADDED: i64 = 0;

    fn make_me_non_dyn_compatible() -> Self;
}

mod private {
    // Not importable from outside the crate, so this lint should not report it.
    pub trait PrivateTrait {
        const ADDED: i64 = 0;

        fn make_me_non_dyn_compatible() -> Self;
    }
}
