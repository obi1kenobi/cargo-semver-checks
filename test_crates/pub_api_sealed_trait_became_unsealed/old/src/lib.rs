#![no_std]

// Traits transitioning from Public API Sealed → Unsealed (Lint should detect these)
pub mod public_api_sealed_to_unsealed {
    #[doc(hidden)]
    pub mod hidden {
        pub trait Sealed {}
        pub struct Token;
    }

    pub trait PublicAPIToBeUnsealed {
        #[doc(hidden)]
        type Hidden;
    }

    pub trait TraitExtendsHiddenPublicAPITrait: hidden::Sealed {}

    pub trait MethodReturnPublicAPIHiddenToken {
        fn method(&self) -> hidden::Token;
    }

    pub trait MethodTakingPublicAPIHiddenToken {
        fn method(&self, token: hidden::Token);
    }
}

// Traits transitioning from Public API Sealed → Unconditionally Sealed (Lint should ignore these)
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
}
