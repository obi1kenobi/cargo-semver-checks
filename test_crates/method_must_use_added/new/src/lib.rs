pub struct StructWithMustUseMethods {}

impl StructWithMustUseMethods {

    // These methods did not have the #[must_use] attribute in the old version.
    // Addition of the attribute should be reported.

    #[must_use]
    pub fn MethodToMustUseMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MethodToMustUseMessageMethod(&self) {}


    // These methods had the #[must_use] attribute in the old version. Changes of
    // the attribute, including deletion, should not be reported.

    pub fn MustUseMethodToMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMethodToMustUseMessageMethod(&self) {}


    // These methods had the #[must_use] attribute in the old version.
    // They also included the user-defined warning message. Changes of
    // the attribute, including deletion, should not be reported.

    pub fn MustUseMessageMethodToMethod(&self) {}

    #[must_use]
    pub fn MustUseMessageMethodToMustUseMethod(&self) {}

    #[must_use = "Baz"]
    pub fn MustUseMessageMethodToMustUseMessageMethod(&self) {}
}


pub struct StructWithMovedProvidedMustUseMethods {}

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

impl TraitWithMovedProvidedMustUseMethods for StructWithMovedProvidedMustUseMethods {}


pub struct StructWithMovedDeclaredMustUseMethods {}

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

impl TraitWithMovedDeclaredMustUseMethods for StructWithMovedDeclaredMustUseMethods {
    
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


// This public struct's inherent method did not have the #[must_use] attribute
// in the old version. Because the method is private, adding the attribute
// should NOT be reported.

pub struct StructWithPrivateMustUseMethods {}

impl StructWithPrivateMustUseMethods {

    #[must_use]
    fn PrivateMethodToPrivateMustUseMethod(&self) {}
}


// This struct is private and adding #[must_use] to it's methods,
// after moving them to a private trait, should NOT be reported.

struct PrivateStructWithMovedMustUseMethods {}

trait PrivateTraitWithMovedMustUseMethods {

    #[must_use]
    fn PrivateStructMethodToMovedPrivateDeclaredMustUseMethod(&self);

    #[must_use]
    fn PrivateStructMethodToMovedPrivateProvidedMustUseMethod(&self) {}

    #[must_use]
    fn PrivateStructMethodToMovedPrivateOverrideMustUseMethod(&self) {}
}

impl PrivateTraitWithMovedMustUseMethods for PrivateStructWithMovedMustUseMethods {

    fn PrivateStructMethodToMovedPrivateDeclaredMustUseMethod(&self) {}

    fn PrivateStructMethodToMovedPrivateOverrideMustUseMethod(&self) {}
}


// This trait is private and adding #[must_use] to it's methods
// should NOT be reported.

trait PrivateTraitWithMustUseMethods {

    #[must_use]
    fn PrivateDeclaredMethodToPrivateDeclaredMustUseMethod(&self);

    #[must_use]
    fn PrivateProvidedMethodToPrivateProvidedMustUseMethod(&self) {}
}


// Adding the #[must_use] attribute inside a trait's impl block
// does NOT generate a compiler lint when calling inner methods
// without using their return value. Thus it should NOT be reported.

pub struct StructWithMovedImplMustUseMethods {}

pub trait TraitWithMovedImplMustUseMethods {

    fn MethodToMovedImplDeclaredMustUseMethod(&self);

    fn MethodToMovedImplOverrideMustUseMethod(&self) {}
}

impl TraitWithMovedImplMustUseMethods for StructWithMovedImplMustUseMethods {
    
    #[must_use]
    fn MethodToMovedImplDeclaredMustUseMethod(&self) {}

    #[must_use]
    fn MethodToMovedImplOverrideMustUseMethod(&self) {}
}


// This struct and it's inherent method were added in the new version of the
// crate, together with the method's attribute. It should NOT be reported
// because adding a new struct is not a breaking change.

pub struct NewStructWithMustUseMethods {}

impl NewStructWithMustUseMethods {

    #[must_use]
    pub fn NewStructMustUseMethod(&self) {}
}


// This struct, trait and it's inherent methods were added in the new version of
// the crate, together with the method's attribute. It should NOT be reported
// because adding a new struct or trait is not a breaking change.

pub struct NewStructWithTraitMustUseMethods {}

pub trait NewTraitWithStructMustUseMethods {

    #[must_use]
    fn NewTraitWithStructDeclaredMustUseMethod(&self);

    #[must_use]
    fn NewTraitWithStructProvidedMustUseMethod(&self) {}

    #[must_use]
    fn NewTraitWithStructOverrideMustUseMethod(&self) {}
}

impl NewTraitWithStructMustUseMethods for NewStructWithTraitMustUseMethods {

    fn NewTraitWithStructDeclaredMustUseMethod(&self) {}

    fn NewTraitWithStructOverrideMustUseMethod(&self) {}
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
