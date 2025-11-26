pub enum EnumWithMustUseMethods {
    Bar,
}

impl EnumWithMustUseMethods {
    // These methods had the #[must_use] attribute in the old version.
    // Removal of the attribute should be reported by this rule.
    #[must_use]
    pub fn MustUseToAssociatedFn() {}

    #[must_use = "Foo"]
    pub fn MustUseMessageToAssociatedFn() {}

    #[must_use]
    pub fn MustUseMethodToMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMethod(&self) {}

    // These methods did not have the #[must_use] attribute in the old version.
    // Addition of the attribute should NOT be reported.

    pub fn MethodToMustUseMethod(&self) {}

    pub fn MethodToMustUseMessageMethod(&self) {}

    // These methods change from one form of #[must_use] to another.
    // They should NOT be reported by this rule.

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMustUseMethod(&self) {}

    pub fn MustUseMethodToMustUseMessageMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMustUseMessageMethod(&self) {}
}

// This public enum's inherent method had the #[must_use] attribute
// in the old version. Because the method is private, deleting the attribute
// should NOT be reported.

pub enum EnumWithPrivateMustUseMethods {
    Bar,
}

impl EnumWithPrivateMustUseMethods {
    #[must_use]
    fn PrivateMustUseMethodToPrivateMethod(&self) {}
}

// This enum is private and deleting #[must_use] from its inherent method
// should NOT be reported.

enum PrivateEnumWithMustUseMethods {
    Bar,
}

impl PrivateEnumWithMustUseMethods {
    #[must_use]
    fn PrivateEnumMustUseMethodToPrivateEnumMethod(&self) {}
}
