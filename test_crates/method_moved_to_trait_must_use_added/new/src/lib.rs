// The role of this test crate is to prevent reporting more than one lint on
// the same change. In this crate, there could be reports about both #[must_use]
// being added to the methods, as well as the methods themselves being moved
// from an impl Struct block to a trait with a respective impl Trait for Struct
// (this is handled by method_moved_to_trait check).
// Because both of these are minor changes, and the #[must_use] added is
// not possible to achieve without the latter check failing, we want to just
// report the latter one (method_moved_to_trait).


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
