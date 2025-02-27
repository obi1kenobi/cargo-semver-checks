// This trait is public API sealed.
pub trait PublicAPIToBeUnsealed {
    #[doc(hidden)]
    type Hidden;
}

#[doc(hidden)]
pub mod hidden_module {
    /// This trait is public-API-sealed because implementing it
    /// requires going outside the public API.
    pub trait HiddenSealed {}

    pub struct Token;
}

/// This trait is public-API-sealed since implementing it requires naming
/// its non-public-API supertrait.
pub trait HiddenSealedInherited: hidden_module::HiddenSealed {}

/// This trait is public-API-sealed because its method's return type is doc-hidden,
/// so external implementers would have to name a non-public-API type to write the impl.
pub trait MethodReturnHiddenSealed {
    fn method(&self) -> hidden_module::Token;
}

/// This trait is public-API-sealed transitively because of its supertrait.
pub trait TransitivelyHiddenSealed: HiddenSealedInherited {}

/// This trait is public-API-sealed, since `Self: hidden_module::HiddenSealed`
/// still requires that `Self` implement a public-API-sealed trait,
/// even though the public-API-sealed trait isn't *exactly* a supertrait.
pub trait HiddenSealedWithWhereSelfBound
where
    Self: hidden_module::HiddenSealed,
{
}

/// This trait is public-API-sealed because its method's argument type is doc-hidden,
/// so external implementers would have to name a non-public-API type to write the impl.
pub trait MethodHiddenSealed {
    fn method(&self, token: hidden_module::Token);
}
