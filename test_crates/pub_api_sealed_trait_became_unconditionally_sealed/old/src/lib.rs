#![no_std]

// Traits transitioning from Public API Sealed → Unconditionally Sealed (Lint should detect these)
pub mod public_api_sealed_to_unconditionally_sealed {
    #[doc(hidden)]
    pub mod hidden {
        pub trait Sealed {}
        pub struct Token;
    }

    pub trait TraitExtendsUnconditionallyHiddenTrait: hidden::Sealed {}

    pub trait MethodReturningUnconditionallyHiddenToken {
        fn method(&self) -> hidden::Token;
    }

    pub trait MethodTakingUnconditionallyHiddenToken {
        fn method(&self, token: hidden::Token);
    }

    pub trait HiddenSealedWithWhereSelfBound
    where
        Self: hidden::Sealed,
    {
    }
}
