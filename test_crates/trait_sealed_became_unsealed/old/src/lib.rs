#[doc(hidden)]
pub mod public_api_hidden_module_tobe_exposed {
    pub trait Sealed {}
    pub struct Token;
}

#[doc(hidden)]
pub mod public_api_hidden_module_to_be_unconditionally_hidden {
    pub trait Sealed {}
    pub struct Token;
}

// Traits transitioning from Public API Sealed → Unsealed (Lint should detect these)
pub trait PublicAPIToBeUnsealed {
    #[doc(hidden)]
    type Hidden;
}
pub trait TraitExtendsPublicAPIHiddenTrait: public_api_hidden_module_tobe_exposed::Sealed {}
pub trait MethodReturnPublicAPIHiddenToken {
    fn method(&self) -> public_api_hidden_module_tobe_exposed::Token;
}
pub trait MethodTakingPublicAPIHiddenToken {
    fn method(&self, token: public_api_hidden_module_tobe_exposed::Token);
}

// Traits transitioning from Public API Sealed → Unconditionally Sealed (Lint should ignore these)
pub trait TraitExtendsUnconditionallyHiddenTrait:
    public_api_hidden_module_to_be_unconditionally_hidden::Sealed
{
}
pub trait MethodReturningUnconditionallyHiddenToken {
    fn method(&self) -> public_api_hidden_module_to_be_unconditionally_hidden::Token;
}
pub trait MethodTakingUnconditionallyHiddenToken {
    fn method(&self, token: public_api_hidden_module_to_be_unconditionally_hidden::Token);
}
