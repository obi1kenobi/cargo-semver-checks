#![no_std]

mod sealed {
    pub(crate) trait Sealed {}
}

// Not sealed and not dyn-compatible. The new method with a default implementation should be
// reported. The new method without one and the new #[doc(hidden)] method should not.
pub trait NotDynCompatible {
    fn make_me_non_dyn_compatible() -> Self;

    fn added_with_default(&self) {}

    fn added_without_default(&self);

    #[doc(hidden)]
    fn added_hidden(&self) {}
}

// Not sealed, and the new method keeps it dyn-compatible. The new method should be reported,
// but the existing method gaining a default implementation should not.
pub trait DynCompatible {
    fn existing(&self) {}

    fn added(&self) {}
}

// Dyn-compatible, but the new generic method makes it no longer dyn-compatible.
// The new method should be reported, and `trait_no_longer_dyn_compatible` also reports the trait.
pub trait BecomesNonDynCompatible {
    fn method(&self);

    fn added<T>(&self, _value: T) {}
}

// Not sealed in the baseline, so the new method should be reported
// even though the trait becomes sealed.
pub trait BecomesSealed: sealed::Sealed {
    fn method(&self);

    fn added(&self) {}
}

// Sealed: `sealed_trait_method_added` reports the new method,
// so this lint should not report it.
pub trait SealedTrait: sealed::Sealed {
    fn method(&self);

    fn added(&self) {}
}

// Sealed from the public API's point of view because of the #[doc(hidden)] associated type:
// `sealed_trait_method_added` reports the new method, so this lint should not report it.
pub trait PublicApiSealed {
    #[doc(hidden)]
    type Hidden;

    fn method(&self);

    fn added(&self) {}
}

// Sealed in the baseline, so this lint should not report the new method
// even though the trait stops being sealed.
pub trait BecomesUnsealed {
    fn method(&self);

    fn added(&self) {}
}

// Not public API, so this lint should not report it.
#[doc(hidden)]
pub trait HiddenTrait {
    fn method(&self);

    fn added(&self) {}
}

mod private {
    // Not importable from outside the crate, so this lint should not report it.
    pub trait PrivateTrait {
        fn method(&self);

        fn added(&self) {}
    }
}
