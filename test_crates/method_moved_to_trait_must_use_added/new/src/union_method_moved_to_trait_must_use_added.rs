pub union UnionWithMovedProvidedMustUseMethods {
    bar: usize,
}

pub trait TraitWithMovedProvidedMustUseMethods {

    // These methods did not have the #[must_use] attribute in the old version.
    // Addition of the attribute should be reported.

    #[must_use]
    fn MethodToMovedProvidedMustUseMethod(&self) {}

    #[must_use = "Foo"]
    fn MethodToMovedProvidedMustUseMessageMethod(&self) {}


    // These methods had the #[must_use] attribute in the old version. Changes of
    // the attribute, including deletion, should not be reported.

    fn MustUseMethodToMovedProvidedMethod(&self) {}

    #[must_use = "Foo"]
    fn MustUseMethodToMovedProvidedMustUseMessageMethod(&self) {}


    // These methods had the #[must_use] attribute in the old version.
    // They also included the user-defined warning message. Changes of
    // the attribute, including deletion, should not be reported.

    fn MustUseMessageMethodToMovedProvidedMethod(&self) {}

    #[must_use]
    fn MustUseMessageMethodToMovedProvidedMustUseMethod(&self) {}

    #[must_use = "Baz"]
    fn MustUseMessageMethodToMovedProvidedMustUseMessageMethod(&self) {}
}

impl TraitWithMovedProvidedMustUseMethods for UnionWithMovedProvidedMustUseMethods {}


pub union UnionWithMovedDeclaredMustUseMethods {
    bar: usize,
}

pub trait TraitWithMovedDeclaredMustUseMethods {

    #[must_use]
    fn MethodToMovedDeclaredMustUseMethod(&self);

    #[must_use = "Foo"]
    fn MethodToMovedDeclaredMustUseMessageMethod(&self);

    fn MustUseMethodToMovedDeclaredMethod(&self);

    #[must_use = "Foo"]
    fn MustUseMethodToMovedDeclaredMustUseMessageMethod(&self);

    fn MustUseMessageMethodToMovedDeclaredMethod(&self);

    #[must_use]
    fn MustUseMessageMethodToMovedDeclaredMustUseMethod(&self);

    #[must_use = "Baz"]
    fn MustUseMessageMethodToMovedDeclaredMustUseMessageMethod(&self);
}

impl TraitWithMovedDeclaredMustUseMethods for UnionWithMovedDeclaredMustUseMethods {
    
    // These methods did not have the #[must_use] attribute in the old version.
    // Addition of the attribute should be reported.

    fn MethodToMovedDeclaredMustUseMethod(&self) {}

    fn MethodToMovedDeclaredMustUseMessageMethod(&self) {}


    // These methods had the #[must_use] attribute in the old version. Changes of
    // the attribute, including deletion, should not be reported.

    fn MustUseMethodToMovedDeclaredMethod(&self) {}

    fn MustUseMethodToMovedDeclaredMustUseMessageMethod(&self) {}


    // These methods had the #[must_use] attribute in the old version.
    // They also included the user-defined warning message. Changes of
    // the attribute, including deletion, should not be reported.

    fn MustUseMessageMethodToMovedDeclaredMethod(&self) {}

    fn MustUseMessageMethodToMovedDeclaredMustUseMethod(&self) {}

    fn MustUseMessageMethodToMovedDeclaredMustUseMessageMethod(&self) {}
}


// This union is private and adding #[must_use] to its methods,
// after moving them to a private trait, should NOT be reported.

union PrivateUnionWithMovedMustUseMethods {
    bar: usize,
}

trait PrivateTraitWithMovedMustUseMethods {

    #[must_use]
    fn PrivateUnionMethodToMovedPrivateDeclaredMustUseMethod(&self);

    #[must_use]
    fn PrivateUnionMethodToMovedPrivateProvidedMustUseMethod(&self) {}

    #[must_use]
    fn PrivateUnionMethodToMovedPrivateOverrideMustUseMethod(&self) {}
}

impl PrivateTraitWithMovedMustUseMethods for PrivateUnionWithMovedMustUseMethods {

    fn PrivateUnionMethodToMovedPrivateDeclaredMustUseMethod(&self) {}

    fn PrivateUnionMethodToMovedPrivateOverrideMustUseMethod(&self) {}
}


// Adding the #[must_use] attribute inside a trait's impl block
// does NOT generate a compiler lint when calling inner methods
// without using their return value. Thus it should NOT be reported.

pub union UnionWithMovedImplMustUseMethods {
    bar: usize,
}

pub trait TraitWithMovedImplMustUseMethods {

    fn MethodToMovedImplDeclaredMustUseMethod(&self);

    fn MethodToMovedImplOverrideMustUseMethod(&self) {}
}

impl TraitWithMovedImplMustUseMethods for UnionWithMovedImplMustUseMethods {
    
    #[must_use]
    fn MethodToMovedImplDeclaredMustUseMethod(&self) {}

    #[must_use]
    fn MethodToMovedImplOverrideMustUseMethod(&self) {}
}


// This union with its inherent methods and the Trait were added in the new
// version of the crate, together with the methods' attributes.
// They should NOT be reported by this rule to avoid duplicate lints.
// They should be reported as new pub types that are part of the crate's API.

pub union NewUnionWithTraitMustUseMethods {
    bar: usize,
}

pub trait NewTraitWithUnionMustUseMethods {

    #[must_use]
    fn NewTraitWithUnionDeclaredMustUseMethod(&self);

    #[must_use]
    fn NewTraitWithUnionProvidedMustUseMethod(&self) {}

    #[must_use]
    fn NewTraitWithUnionOverrideMustUseMethod(&self) {}
}

impl NewTraitWithUnionMustUseMethods for NewUnionWithTraitMustUseMethods {

    fn NewTraitWithUnionDeclaredMustUseMethod(&self) {}

    fn NewTraitWithUnionOverrideMustUseMethod(&self) {}
}
