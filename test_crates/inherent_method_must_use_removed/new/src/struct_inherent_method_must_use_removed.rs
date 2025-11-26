pub struct StructWithMustUseMethods {}

impl StructWithMustUseMethods {
    // These methods had the #[must_use] attribute in the old version.
    // Removal of the attribute should be reported by this rule.
    pub fn MustUseToAssociatedFn() {}

    pub fn MustUseMessageToAssociatedFn() {}

    pub fn MustUseMethodToMethod(&self) {}

    pub fn MustUseMessageMethodToMethod(&self) {}

    // These methods did not have the #[must_use] attribute in the old version.
    // Addition of the attribute should NOT be reported.

    #[must_use]
    pub fn MethodToMustUseMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MethodToMustUseMessageMethod(&self) {}

    // These methods change from one form of #[must_use] to another.
    // They should NOT be reported by this rule.

    #[must_use]
    pub fn MustUseMessageMethodToMustUseMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMethodToMustUseMessageMethod(&self) {}

    #[must_use = "Baz"]
    pub fn MustUseMessageMethodToMustUseMessageMethod(&self) {}
}

// This public struct's inherent method had the #[must_use] attribute
// in the old version. Because the method is private, deleting the attribute
// should NOT be reported.

pub struct StructWithPrivateMustUseMethods {}

impl StructWithPrivateMustUseMethods {
    fn PrivateMustUseMethodToPrivateMethod(&self) {}
}

// This struct is private and deleting #[must_use] from its inherent method
// should NOT be reported.

struct PrivateStructWithMustUseMethods {}

impl PrivateStructWithMustUseMethods {
    fn PrivateStructMustUseMethodToPrivateStructMethod(&self) {}
}
