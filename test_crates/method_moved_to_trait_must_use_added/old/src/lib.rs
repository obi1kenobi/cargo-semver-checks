// The role of this test crate is to prevent reporting more than one lint on
// the same change. In this crate, there could be reports about both #[must_use]
// being added to the methods, as well as the methods themselves being moved
// from an impl Struct block to a trait with a respective impl Trait for Struct
// (this is handled by method_moved_to_trait check).
// Because both of these are minor changes, and the #[must_use] added is
// not possible to achieve without the latter check failing, we want to just
// report the latter one (method_moved_to_trait).


pub struct StructWithMovedProvidedMustUseMethods {}

impl StructWithMovedProvidedMustUseMethods {

    // These methods did not have the #[must_use] attribute in the old version.
    // Addition of the attribute should be reported.

    pub fn MethodToMovedProvidedMustUseMethod(&self) {}

    pub fn MethodToMovedProvidedMustUseMessageMethod(&self) {}


    // These methods had the #[must_use] attribute in the old version. Changes
    // of the attribute, including deletion, should not be reported.

    #[must_use]
    pub fn MustUseMethodToMovedProvidedMethod(&self) {}

    #[must_use]
    pub fn MustUseMethodToMovedProvidedMustUseMessageMethod(&self) {}


    // These methods had the #[must_use] attribute in the old version.
    // They also included the user-defined warning message. Changes of
    // the attribute, including deletion, should not be reported.

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMovedProvidedMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMovedProvidedMustUseMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMovedProvidedMustUseMessageMethod(&self) {}
}

pub trait TraitWithMovedProvidedMustUseMethods {}


pub struct StructWithMovedDeclaredMustUseMethods {}

impl StructWithMovedDeclaredMustUseMethods {

    // These methods did not have the #[must_use] attribute in the old version.
    // Addition of the attribute should be reported.

    pub fn MethodToMovedDeclaredMustUseMethod(&self) {}

    pub fn MethodToMovedDeclaredMustUseMessageMethod(&self) {}


    // These methods had the #[must_use] attribute in the old version. Changes
    // of the attribute, including deletion, should not be reported.

    #[must_use]
    pub fn MustUseMethodToMovedDeclaredMethod(&self) {}

    #[must_use]
    pub fn MustUseMethodToMovedDeclaredMustUseMessageMethod(&self) {}


    // These methods had the #[must_use] attribute in the old version.
    // They also included the user-defined warning message. Changes of
    // the attribute, including deletion, should not be reported.

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMovedDeclaredMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMovedDeclaredMustUseMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMovedDeclaredMustUseMessageMethod(&self) {}
}

pub trait TraitWithMovedDeclaredMustUseMethods {}

// This struct is private and adding #[must_use] to it's methods,
// after moving them to a private trait, should NOT be reported.

struct PrivateStructWithMovedMustUseMethods {}

impl PrivateStructWithMovedMustUseMethods {
    
    pub fn PrivateStructMethodToMovedPrivateDeclaredMustUseMethod(&self) {}

    pub fn PrivateStructMethodToMovedPrivateProvidedMustUseMethod(&self) {}

    pub fn PrivateStructMethodToMovedPrivateOverrideMustUseMethod(&self) {}
}

trait PrivateTraitWithMovedMustUseMethods {}


// Adding the #[must_use] attribute inside a trait's impl block
// does NOT generate a compiler lint when calling inner methods
// without using the return value. Thus it should NOT be reported.

pub struct StructWithMovedImplMustUseMethods {}

impl StructWithMovedImplMustUseMethods {

    pub fn MethodToMovedImplDeclaredMustUseMethod(&self) {}

    pub fn MethodToMovedImplOverrideMustUseMethod(&self) {}
}

pub trait TraitWithMovedImplMustUseMethods {}
