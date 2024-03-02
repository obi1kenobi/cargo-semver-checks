pub enum EnumWithMustUseMethods {
    Bar,
}

impl EnumWithMustUseMethods {
    // These methods did not have the #[must_use] attribute in the old version.
    // Addition of the attribute should be reported.
    #[must_use]
    pub fn AssociatedFnToMustUse() {}

    #[must_use = "Foo"]
    pub fn AssociatedFnToMustUseMessage() {}

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

// This public enum's inherent method did not have the #[must_use] attribute
// in the old version. Because the method is private, adding the attribute
// should NOT be reported.

pub enum EnumWithPrivateMustUseMethods {
    Bar,
}

impl EnumWithPrivateMustUseMethods {
    #[must_use]
    fn PrivateMethodToPrivateMustUseMethod(&self) {}
}

// This enum is private and adding #[must_use] to its inherent method
// should NOT be reported.

enum PrivateEnumWithMustUseMethods {
    Bar,
}

impl PrivateEnumWithMustUseMethods {
    #[must_use]
    fn PrivateEnumMethodToPrivateEnumMustUseMethod(&self) {}
}

// This enum and its inherent method were added in the new version of the
// crate, together with the method's attribute.
// It should NOT be reported by this rule to avoid duplicate lints.
// It should be reported as a new pub type that is part of the crate's API.

pub enum NewEnumWithMustUseMethods {
    Bar,
}

impl NewEnumWithMustUseMethods {
    #[must_use]
    pub fn NewEnumMustUseAssociatedFn() {}

    #[must_use]
    pub fn NewEnumMustUseMethod(&self) {}
}
