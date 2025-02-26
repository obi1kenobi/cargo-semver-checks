// This trait is public API sealed.
pub trait PublicAPIToBeUnsealed {
    type Hidden;
}

pub mod hidden_module {
    /// This trait is public-API-sealed because implementing it
    /// requires going outside the public API.
    pub trait HiddenSealed {}

    pub struct Token;
}

/// This trait is public-API-sealed since implementing it requires naming
/// its non-public-API supertrait.
pub trait HiddenSealedInherited: hidden_module::HiddenSealed {}
