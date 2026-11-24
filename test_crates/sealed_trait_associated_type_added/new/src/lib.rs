#![no_std]
#![feature(associated_type_defaults)]

mod sealed {
    pub(crate) trait Sealed {}
}

// Sealed and not dyn-compatible. The new types, with and without a default value,
// should be reported. The new #[doc(hidden)] type should not.
pub trait SealedNotDynCompatible: sealed::Sealed {
    type AddedWithoutDefault;
    type AddedWithDefault = bool;
    #[doc(hidden)]
    type AddedHidden;

    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed from the public API's point of view because of the #[doc(hidden)] associated type.
// The new type should be reported.
pub trait PublicApiSealedNotDynCompatible {
    #[doc(hidden)]
    type Hidden;

    type Added;

    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed in the baseline, so the new type should be reported
// even though the trait stops being sealed.
pub trait BecomesUnsealed {
    type Added;

    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed and not dyn-compatible in the baseline. The new type should be reported
// even though the trait becomes dyn-compatible.
pub trait BecomesDynCompatible: sealed::Sealed {
    type Added;

    fn make_me_non_dyn_compatible() -> Self
    where
        Self: Sized;
}

// Sealed and dyn-compatible, and the new type keeps it dyn-compatible.
// The new type should be reported.
pub trait SealedDynCompatible: sealed::Sealed {
    type Added;

    fn method(&self);
}

// Not sealed: `trait_associated_type_added` reports the new type without a default value,
// so this lint should not report either new type.
pub trait NotSealed {
    type AddedWithoutDefault;
    type AddedWithDefault = bool;

    fn make_me_non_dyn_compatible() -> Self;
}

// Not sealed in the baseline, so this lint should not report the new type
// even though the trait becomes sealed.
pub trait BecomesSealed: sealed::Sealed {
    type Added;

    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed and dyn-compatible, but the new generic associated type makes it
// no longer dyn-compatible. The new type should be reported,
// and `trait_no_longer_dyn_compatible` also reports the trait.
pub trait BecomesNonDynCompatible: sealed::Sealed {
    type Added<T>;

    fn method(&self);
}

// Not public API, so this lint should not report it.
#[doc(hidden)]
pub trait HiddenTrait: sealed::Sealed {
    type Added;

    fn make_me_non_dyn_compatible() -> Self;
}

// No longer public API, so this lint should not report the new type.
// `trait_now_doc_hidden` reports the trait instead.
#[doc(hidden)]
pub trait BecomesHidden: sealed::Sealed {
    type Added;

    fn make_me_non_dyn_compatible() -> Self;
}

// Not public in the baseline, so this lint should not report the new type
// even though the trait becomes public.
pub trait BecomesPublic: sealed::Sealed {
    type Added;

    fn make_me_non_dyn_compatible() -> Self;
}

mod private {
    // Not importable from outside the crate, so this lint should not report it.
    pub trait PrivateTrait: super::sealed::Sealed {
        type Added;

        fn make_me_non_dyn_compatible() -> Self;
    }
}
