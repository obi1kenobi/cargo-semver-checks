// This test file pair contains test cases against items changing the type
// between the old and new versions while their names and inherent methods
// remain unchanged.
// This can result in false-positives by reporting an addition of #[must_use]
// to an inherent method while the ImplOwner of the method has changed, making
// the new version method no more related to the old version method.

pub struct EnumToStructWithMustUseMethods {}

impl EnumToStructWithMustUseMethods {
    #[must_use]
    pub fn AssociatedFnToMustUse() {}

    #[must_use = "Foo"]
    pub fn AssociatedFnToMustUseMessage() {}

    #[must_use]
    pub fn MethodToMustUseMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MethodToMustUseMessageMethod(&self) {}

    pub fn MustUseMethodToMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMethodToMustUseMessageMethod(&self) {}

    pub fn MustUseMessageMethodToMethod(&self) {}

    #[must_use]
    pub fn MustUseMessageMethodToMustUseMethod(&self) {}

    #[must_use = "Baz"]
    pub fn MustUseMessageMethodToMustUseMessageMethod(&self) {}
}

pub union EnumToUnionWithMustUseMethods {
    bar: usize,
}

impl EnumToUnionWithMustUseMethods {
    #[must_use]
    pub fn AssociatedFnToMustUse() {}

    #[must_use = "Foo"]
    pub fn AssociatedFnToMustUseMessage() {}

    #[must_use]
    pub fn MethodToMustUseMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MethodToMustUseMessageMethod(&self) {}

    pub fn MustUseMethodToMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMethodToMustUseMessageMethod(&self) {}

    pub fn MustUseMessageMethodToMethod(&self) {}

    #[must_use]
    pub fn MustUseMessageMethodToMustUseMethod(&self) {}

    #[must_use = "Baz"]
    pub fn MustUseMessageMethodToMustUseMessageMethod(&self) {}
}

pub enum StructToEnumWithMustUseMethods {
    Bar,
}

impl StructToEnumWithMustUseMethods {
    #[must_use]
    pub fn AssociatedFnToMustUse() {}

    #[must_use = "Foo"]
    pub fn AssociatedFnToMustUseMessage() {}

    #[must_use]
    pub fn MethodToMustUseMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MethodToMustUseMessageMethod(&self) {}

    pub fn MustUseMethodToMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMethodToMustUseMessageMethod(&self) {}

    pub fn MustUseMessageMethodToMethod(&self) {}

    #[must_use]
    pub fn MustUseMessageMethodToMustUseMethod(&self) {}

    #[must_use = "Baz"]
    pub fn MustUseMessageMethodToMustUseMessageMethod(&self) {}
}

pub union StructToUnionWithMustUseMethods {
    bar: usize,
}

impl StructToUnionWithMustUseMethods {
    #[must_use]
    pub fn AssociatedFnToMustUse() {}

    #[must_use = "Foo"]
    pub fn AssociatedFnToMustUseMessage() {}

    #[must_use]
    pub fn MethodToMustUseMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MethodToMustUseMessageMethod(&self) {}

    pub fn MustUseMethodToMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMethodToMustUseMessageMethod(&self) {}

    pub fn MustUseMessageMethodToMethod(&self) {}

    #[must_use]
    pub fn MustUseMessageMethodToMustUseMethod(&self) {}

    #[must_use = "Baz"]
    pub fn MustUseMessageMethodToMustUseMessageMethod(&self) {}
}

pub enum UnionToEnumWithMustUseMethods {
    Bar,
}

impl UnionToEnumWithMustUseMethods {
    #[must_use]
    pub fn AssociatedFnToMustUse() {}

    #[must_use = "Foo"]
    pub fn AssociatedFnToMustUseMessage() {}

    #[must_use]
    pub fn MethodToMustUseMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MethodToMustUseMessageMethod(&self) {}

    pub fn MustUseMethodToMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMethodToMustUseMessageMethod(&self) {}

    pub fn MustUseMessageMethodToMethod(&self) {}

    #[must_use]
    pub fn MustUseMessageMethodToMustUseMethod(&self) {}

    #[must_use = "Baz"]
    pub fn MustUseMessageMethodToMustUseMessageMethod(&self) {}
}

pub struct UnionToStructWithMustUseMethods {}

impl UnionToStructWithMustUseMethods {
    #[must_use]
    pub fn AssociatedFnToMustUse() {}

    #[must_use = "Foo"]
    pub fn AssociatedFnToMustUseMessage() {}

    #[must_use]
    pub fn MethodToMustUseMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MethodToMustUseMessageMethod(&self) {}

    pub fn MustUseMethodToMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMethodToMustUseMessageMethod(&self) {}

    pub fn MustUseMessageMethodToMethod(&self) {}

    #[must_use]
    pub fn MustUseMessageMethodToMustUseMethod(&self) {}

    #[must_use = "Baz"]
    pub fn MustUseMessageMethodToMustUseMessageMethod(&self) {}
}
