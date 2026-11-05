#![no_std]

mod sealed {
    pub(crate) trait Sealed {}
}

// Sealed and dyn-compatible, and the new methods keep it dyn-compatible. The new methods,
// with and without a default implementation, should be reported.
// The new #[doc(hidden)] method should not.
pub trait SealedDynCompatible: sealed::Sealed {
    fn method(&self);

    fn added_without_default(&self);

    fn added_with_default(&self) {}

    #[doc(hidden)]
    fn added_hidden(&self) {}
}

// Sealed and not dyn-compatible. The new method should be reported.
pub trait SealedNotDynCompatible: sealed::Sealed {
    fn make_me_non_dyn_compatible() -> Self;

    fn added(&self);
}

// Sealed from the public API's point of view because of the #[doc(hidden)] associated type.
// The new method should be reported.
pub trait PublicApiSealed {
    #[doc(hidden)]
    type Hidden;

    fn method(&self);

    fn added(&self);
}

// Sealed in the baseline, so the new method should be reported
// even though the trait stops being sealed.
pub trait BecomesUnsealed {
    fn method(&self);

    fn added(&self) {}
}

// Sealed and dyn-compatible, but the new generic method makes it no longer dyn-compatible.
// The new method should be reported, and `trait_no_longer_dyn_compatible` also reports the trait.
pub trait BecomesNonDynCompatible: sealed::Sealed {
    fn method(&self);

    fn added<T>(&self, value: T);
}

// Not sealed: `trait_method_added` reports the new method without a default implementation,
// so this lint should not report either new method.
pub trait NotSealed {
    fn method(&self);

    fn added_without_default(&self);

    fn added_with_default(&self) {}
}

// Not sealed in the baseline, so this lint should not report the new method
// even though the trait becomes sealed.
pub trait BecomesSealed: sealed::Sealed {
    fn method(&self);

    fn added(&self) {}
}

// Not public API, so this lint should not report it.
#[doc(hidden)]
pub trait HiddenTrait: sealed::Sealed {
    fn method(&self);

    fn added(&self);
}

mod private {
    // Not importable from outside the crate, so this lint should not report it.
    pub trait PrivateTrait: super::sealed::Sealed {
        fn method(&self);

        fn added(&self);
    }
}
