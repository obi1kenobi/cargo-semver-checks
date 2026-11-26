#![no_std]
#![feature(associated_type_defaults)]

mod sealed {
    pub(crate) trait Sealed {}
}

// Not sealed and not dyn-compatible. The new type with a default value should be reported.
// The new type without a default value and the new #[doc(hidden)] type should not.
pub trait NotDynCompatible {
    type AddedWithDefault = bool;

    type AddedWithoutDefault;

    #[doc(hidden)]
    type AddedHidden = bool;

    fn make_me_non_dyn_compatible() -> Self;
}

// Not sealed, and the new type keeps it dyn-compatible. The new type should be reported,
// but the existing type gaining a default value should not.
pub trait DynCompatible {
    type Existing = bool;

    type Added = bool;

    fn method(&self);
}

// Dyn-compatible, but the new generic associated type makes it no longer dyn-compatible.
// The new type should be reported, and `trait_no_longer_dyn_compatible` also reports the trait.
pub trait BecomesNonDynCompatible {
    type Added<T> = T;

    fn method(&self);
}

// Not sealed in the baseline, so the new type should be reported
// even though the trait becomes sealed.
pub trait BecomesSealed: sealed::Sealed {
    type Added = bool;

    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed, so this lint should not report the new type.
pub trait SealedTrait: sealed::Sealed {
    type Added = bool;

    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed from the public API's point of view because of the #[doc(hidden)] associated type
// without a default value, so this lint should not report the new type.
pub trait PublicApiSealed {
    #[doc(hidden)]
    type Hidden;

    type Added = bool;

    fn make_me_non_dyn_compatible() -> Self;
}

// Sealed in the baseline, so this lint should not report the new type
// even though the trait stops being sealed.
pub trait BecomesUnsealed {
    type Added = bool;

    fn make_me_non_dyn_compatible() -> Self;
}

// Not public API, so this lint should not report it.
#[doc(hidden)]
pub trait HiddenTrait {
    type Added = bool;

    fn make_me_non_dyn_compatible() -> Self;
}

// No longer public API, so this lint should not report the new type.
// `trait_now_doc_hidden` reports the trait instead.
#[doc(hidden)]
pub trait BecomesHidden {
    type Added = bool;

    fn make_me_non_dyn_compatible() -> Self;
}

// Not public in the baseline, so this lint should not report the new type
// even though the trait becomes public.
pub trait BecomesPublic {
    type Added = bool;

    fn make_me_non_dyn_compatible() -> Self;
}

mod private {
    // Not importable from outside the crate, so this lint should not report it.
    pub trait PrivateTrait {
        type Added = bool;

        fn make_me_non_dyn_compatible() -> Self;
    }
}

pub trait Reexported {
    // Unhiding an existing type should not report a type addition.
    type PreviouslyHidden = bool;

    // Only Reexported::Added should be reported. Each alias is hidden in one version.
    type Added = bool;

    fn make_me_non_dyn_compatible() -> Self;
}

pub use Reexported as NewlyVisibleAlias;
#[doc(hidden)]
pub use Reexported as NewlyHiddenAlias;
