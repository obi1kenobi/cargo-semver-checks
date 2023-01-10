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


// This trait is private and adding #[must_use] to its methods
// should NOT be reported.

trait PrivateTraitWithMustUseMethods {

    #[must_use]
    fn PrivateDeclaredMethodToPrivateDeclaredMustUseMethod(&self);

    #[must_use]
    fn PrivateProvidedMethodToPrivateProvidedMustUseMethod(&self) {}
}


// This trait and its methods were added in the new version of the crate,
// together with the methods' attributes.
// It should NOT be reported by this rule to avoid duplicate lints.
// It should be reported as a new pub type that is part of the crate's API.

pub trait NewTraitWithMustUseMethods {

    #[must_use]
    fn NewTraitWithDeclaredMustUseMethod(&self);

    #[must_use]
    fn NewTraitWithProvidedMustUseMethod(&self) {}
}
