#![no_std]
#![feature(associated_type_defaults)]

mod sealed {
    pub(crate) trait Sealed {}
}

// Dyn-compatible in both versions, so both new types should be reported: `dyn DynCompatible`
// must now specify them, even the one with a default value. The type without a default value
// is also reported by `trait_associated_type_added`, since the trait isn't sealed.
pub trait DynCompatible {
    type AddedWithoutDefault;
    type AddedWithDefault = bool;

    fn method(&self);
}

// Sealing doesn't matter for `dyn Trait` usage, so the new types should be reported,
// including the #[doc(hidden)] one, since it must be specified as well.
pub trait SealedDynCompatible: sealed::Sealed {
    type Added;
    #[doc(hidden)]
    type AddedHidden;

    fn method(&self);
}

// Already had an associated type, so downstream code used it as `dyn Trait<Existing = ...>`.
// Only the new type should be reported.
pub trait ExistingAssociatedType: sealed::Sealed {
    type Existing;
    type Added;

    fn method(&self);
}

// TODO: A new type with a `where Self: Sized` bound doesn't need to be specified in `dyn Trait`,
// so this shouldn't be reported. It's currently reported because the schema doesn't
// expose associated type where clauses yet.
pub trait SelfSizedAssociatedType: sealed::Sealed {
    type Added
    where
        Self: Sized;

    fn method(&self);
}

// The new generic associated type makes the trait no longer dyn-compatible.
// `trait_no_longer_dyn_compatible` reports that, so this lint should not.
pub trait BecomesNotDynCompatible: sealed::Sealed {
    type Added<T>;

    fn method(&self);
}

// Not dyn-compatible in the baseline, so no downstream code could have used it as `dyn Trait`.
// This lint should not report the new type.
pub trait NotDynCompatible {
    type Added = bool;

    fn make_me_non_dyn_compatible() -> Self;
}

// Not dyn-compatible in the baseline, so this lint should not report the new type
// even though the trait becomes dyn-compatible.
pub trait BecomesDynCompatible {
    type Added = bool;

    fn make_me_non_dyn_compatible() -> Self
    where
        Self: Sized;
}

// Not public API, so this lint should not report it.
#[doc(hidden)]
pub trait HiddenTrait: sealed::Sealed {
    type Added;

    fn method(&self);
}

mod private {
    // Not importable from outside the crate, so this lint should not report it.
    pub trait PrivateTrait: super::sealed::Sealed {
        type Added;

        fn method(&self);
    }
}
