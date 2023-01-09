pub trait TraitWithDeclaredMustUseMethods {

    // These methods did not have the #[must_use] attribute in the old version.
    // Addition of the attribute should be reported.

    #[must_use]
    fn DeclaredMethodToDeclaredMustUseMethod(&self);

    #[must_use = "Foo"]
    fn DeclaredMethodToDeclaredMustUseMessageMethod(&self);


    // These methods had the #[must_use] attribute in the old version. Changes of
    // the attribute, including deletion, should not be reported.

    fn DeclaredMustUseMethodToDeclaredMethod(&self);

    #[must_use = "Foo"]
    fn DeclaredMustUseMethodToDeclaredMustUseMessageMethod(&self);


    // These methods had the #[must_use] attribute in the old version.
    // They also included the user-defined warning message. Changes of
    // the attribute, including deletion, should not be reported.

    fn DeclaredMustUseMessageMethodToDeclaredMethod(&self);

    #[must_use]
    fn DeclaredMustUseMessageMethodToDeclaredMustUseMethod(&self);

    #[must_use = "Baz"]
    fn DeclaredMustUseMessageMethodToDeclaredMustUseMessageMethod(&self);
}


pub trait TraitWithProvidedMustUseMethods {

    // These methods did not have the #[must_use] attribute in the old version.
    // Addition of the attribute should be reported.

    #[must_use]
    fn ProvidedMethodToProvidedMustUseMethod(&self) {}

    #[must_use = "Foo"]
    fn ProvidedMethodToProvidedMustUseMessageMethod(&self) {}


    // These methods had the #[must_use] attribute in the old version. Changes of
    // the attribute, including deletion, should not be reported.

    fn ProvidedMustUseMethodToProvidedMethod(&self) {}

    #[must_use = "Foo"]
    fn ProvidedMustUseMethodToProvidedMustUseMessageMethod(&self) {}


    // These methods had the #[must_use] attribute in the old version.
    // They also included the user-defined warning message. Changes of
    // the attribute, including deletion, should not be reported.

    fn ProvidedMustUseMessageMethodToProvidedMethod(&self) {}

    #[must_use]
    fn ProvidedMustUseMessageMethodToProvidedMustUseMethod(&self) {}

    #[must_use = "Baz"]
    fn ProvidedMustUseMessageMethodToProvidedMustUseMessageMethod(&self) {}
}


// This trait is private and adding #[must_use] to it's methods
// should NOT be reported.

trait PrivateTraitWithMustUseMethods {

    #[must_use]
    fn PrivateDeclaredMethodToPrivateDeclaredMustUseMethod(&self);

    #[must_use]
    fn PrivateProvidedMethodToPrivateProvidedMustUseMethod(&self) {}
}


// This trait and it's methods were added in the new version of the crate,
// together with the method's attribute. It should NOT be reported because
// adding a new trait is not a breaking change.

pub trait NewTraitWithMustUseMethods {

    #[must_use]
    fn NewTraitWithDeclaredMustUseMethod(&self);

    #[must_use]
    fn NewTraitWithProvidedMustUseMethod(&self) {}
}
