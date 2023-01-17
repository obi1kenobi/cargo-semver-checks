pub trait TraitWithDeclaredMustUseMethods {

    // These methods did not have the #[must_use] attribute in the old version.
    // Addition of the attribute should be reported.

    fn DeclaredMethodToDeclaredMustUseMethod(&self);

    fn DeclaredMethodToDeclaredMustUseMessageMethod(&self);


    // These methods had the #[must_use] attribute in the old version. Changes
    // of the attribute, including deletion, should not be reported.

    #[must_use]
    fn DeclaredMustUseMethodToDeclaredMethod(&self);

    #[must_use]
    fn DeclaredMustUseMethodToDeclaredMustUseMessageMethod(&self);


    // These methods had the #[must_use] attribute in the old version.
    // They also included the user-defined warning message. Changes of
    // the attribute, including deletion, should not be reported.

    #[must_use = "Foo"]
    fn DeclaredMustUseMessageMethodToDeclaredMethod(&self);

    #[must_use = "Foo"]
    fn DeclaredMustUseMessageMethodToDeclaredMustUseMethod(&self);

    #[must_use = "Foo"]
    fn DeclaredMustUseMessageMethodToDeclaredMustUseMessageMethod(&self);
}


pub trait TraitWithProvidedMustUseMethods {

    // These methods did not have the #[must_use] attribute in the old version.
    // Addition of the attribute should be reported.

    fn ProvidedMethodToProvidedMustUseMethod(&self) {}

    fn ProvidedMethodToProvidedMustUseMessageMethod(&self) {}


    // These methods had the #[must_use] attribute in the old version. Changes
    // of the attribute, including deletion, should not be reported.

    #[must_use]
    fn ProvidedMustUseMethodToProvidedMethod(&self) {}

    #[must_use]
    fn ProvidedMustUseMethodToProvidedMustUseMessageMethod(&self) {}


    // These methods had the #[must_use] attribute in the old version.
    // They also included the user-defined warning message. Changes of
    // the attribute, including deletion, should not be reported.

    #[must_use = "Foo"]
    fn ProvidedMustUseMessageMethodToProvidedMethod(&self) {}

    #[must_use = "Foo"]
    fn ProvidedMustUseMessageMethodToProvidedMustUseMethod(&self) {}

    #[must_use = "Foo"]
    fn ProvidedMustUseMessageMethodToProvidedMustUseMessageMethod(&self) {}
}


pub trait TraitWithDeclaredToProvidedMustUseMethods {

    // These methods did not have the #[must_use] attribute in the old version.
    // Addition of the attribute should be reported.

    fn DeclaredMethodToProvidedMustUseMethod(&self);

    fn DeclaredMethodToProvidedMustUseMessageMethod(&self);


    // These methods had the #[must_use] attribute in the old version. Changes
    // of the attribute, including deletion, should not be reported.

    #[must_use]
    fn DeclaredMustUseMethodToProvidedMethod(&self);

    #[must_use]
    fn DeclaredMustUseMethodToProvidedMustUseMessageMethod(&self);


    // These methods had the #[must_use] attribute in the old version.
    // They also included the user-defined warning message. Changes of
    // the attribute, including deletion, should not be reported.

    #[must_use = "Foo"]
    fn DeclaredMustUseMessageMethodToProvidedMethod(&self);

    #[must_use = "Foo"]
    fn DeclaredMustUseMessageMethodToProvidedMustUseMethod(&self);

    #[must_use = "Foo"]
    fn DeclaredMustUseMessageMethodToProvidedMustUseMessageMethod(&self);
}


// This trait's methods were provided in the old version of the crate, but
// their bodies have been removed turning them into declared methods.
// They should NOT be reported by this rule to avoid duplicate lints.
// They should be reported as changes of the trait's provided methods into
// declared methods.

pub trait TraitWithProvidedToDeclaredMustUseMethods {

    fn ProvidedMethodToDeclaredMustUseMethod(&self) {}

    fn ProvidedMethodToDeclaredMustUseMessageMethod(&self) {}

    #[must_use]
    fn ProvidedMustUseMethodToDeclaredMethod(&self) {}

    #[must_use]
    fn ProvidedMustUseMethodToDeclaredMustUseMessageMethod(&self) {}

    #[must_use = "Foo"]
    fn ProvidedMustUseMessageMethodToDeclaredMethod(&self) {}

    #[must_use = "Foo"]
    fn ProvidedMustUseMessageMethodToDeclaredMustUseMethod(&self) {}

    #[must_use = "Foo"]
    fn ProvidedMustUseMessageMethodToDeclaredMustUseMessageMethod(&self) {}
}


// This trait is private and adding #[must_use] to its methods
// should NOT be reported.

trait PrivateTraitWithMustUseMethods {

    fn PrivateDeclaredMethodToPrivateDeclaredMustUseMethod(&self);

    fn PrivateProvidedMethodToPrivateProvidedMustUseMethod(&self) {}

    fn PrivateDeclaredMethodToPrivateProvidedMustUseMethod(&self);

    fn PrivateProvidedMethodToPrivateDeclaredMustUseMethod(&self) {}
}
