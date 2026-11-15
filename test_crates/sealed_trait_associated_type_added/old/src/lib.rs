#![no_std]
#![feature(associated_type_defaults)]

mod sealed {
    pub(crate) trait Sealed {}
}

// Sealed and not dyn-compatible. The new types, with and without a default value,
// should be reported. The new #[doc(hidden)] type should not.
pub trait SealedNotDynCompatible: sealed::Sealed {
    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed from the public API's point of view because of the #[doc(hidden)] associated type.
// The new type should be reported.
pub trait PublicApiSealedNotDynCompatible {
    #[doc(hidden)]
    type Hidden;

    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed in the baseline, so the new type should be reported
// even though the trait stops being sealed.
pub trait BecomesUnsealed: sealed::Sealed {
    fn make_me_non_dyn_compatible() -> Self;
}

// Not dyn-compatible in the baseline, so no downstream code could have used it as `dyn Trait`.
// The new type should be reported even though the trait becomes dyn-compatible.
pub trait BecomesDynCompatible: sealed::Sealed {
    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed but dyn-compatible: downstream `dyn SealedDynCompatible` stops compiling
// because it must now specify the new type, so this lint should not report it.
pub trait SealedDynCompatible: sealed::Sealed {
    fn method(&self);
}

// Not sealed: `trait_associated_type_added` reports the new type without a default value,
// so this lint should not report either new type.
pub trait NotSealed {
    fn make_me_non_dyn_compatible() -> Self;
}

// Not sealed in the baseline, so this lint should not report the new type
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
