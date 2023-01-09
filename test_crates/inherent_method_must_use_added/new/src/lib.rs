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


// This public struct's inherent method did not have the #[must_use] attribute
// in the old version. Because the method is private, adding the attribute
// should NOT be reported.

pub struct StructWithPrivateMustUseMethods {}

impl StructWithPrivateMustUseMethods {

    #[must_use]
    fn PrivateMethodToPrivateMustUseMethod(&self) {}
}


// This struct is private and adding #[must_use] to it's inherent methods
// should NOT be reported.

struct PrivateStructWithMustUseMethods {}

impl PrivateStructWithMustUseMethods {

    #[must_use]
    fn PrivateStructMethodToPrivateStructMustUseMethod(&self) {}
}


// This struct and it's inherent method were added in the new version of the
// crate, together with the method's attribute. It should NOT be reported
// because adding a new struct is not a breaking change.

pub struct NewStructWithMustUseMethods {}

impl NewStructWithMustUseMethods {

    #[must_use]
    pub fn NewStructMustUseMethod(&self) {}
}
