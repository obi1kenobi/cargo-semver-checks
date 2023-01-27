// This test file pair contains test cases against items changing the type
// between the old and new versions while their names and inherent methods
// remain unchanged.
// This can result in false-positives by reporting an addition of #[must_use]
// to an inherent method while the ImplOwner of the method has changed, making
// the new version method no more related to the old version method.

pub enum EnumToStructWithMustUseMethods {
    Bar,
}

impl EnumToStructWithMustUseMethods {

    pub fn MethodToMustUseMethod(&self) {}

    pub fn MethodToMustUseMessageMethod(&self) {}

    #[must_use]
    pub fn MustUseMethodToMethod(&self) {}

    #[must_use]
    pub fn MustUseMethodToMustUseMessageMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMustUseMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMustUseMessageMethod(&self) {}
}


pub enum EnumToUnionWithMustUseMethods {
    Bar,
}

impl EnumToUnionWithMustUseMethods {

    pub fn MethodToMustUseMethod(&self) {}

    pub fn MethodToMustUseMessageMethod(&self) {}

    #[must_use]
    pub fn MustUseMethodToMethod(&self) {}

    #[must_use]
    pub fn MustUseMethodToMustUseMessageMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMustUseMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMustUseMessageMethod(&self) {}
}


pub struct StructToEnumWithMustUseMethods {
    internal: bool,
}

impl StructToEnumWithMustUseMethods {

    pub fn MethodToMustUseMethod(&self) {}

    pub fn MethodToMustUseMessageMethod(&self) {}

    #[must_use]
    pub fn MustUseMethodToMethod(&self) {}

    #[must_use]
    pub fn MustUseMethodToMustUseMessageMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMustUseMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMustUseMessageMethod(&self) {}
}


pub struct StructToUnionWithMustUseMethods {
    internal: bool,
}

impl StructToUnionWithMustUseMethods {

    pub fn MethodToMustUseMethod(&self) {}

    pub fn MethodToMustUseMessageMethod(&self) {}

    #[must_use]
    pub fn MustUseMethodToMethod(&self) {}

    #[must_use]
    pub fn MustUseMethodToMustUseMessageMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMustUseMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMustUseMessageMethod(&self) {}
}


pub union UnionToEnumWithMustUseMethods {
    bar: usize,
}

impl UnionToEnumWithMustUseMethods {

    pub fn MethodToMustUseMethod(&self) {}

    pub fn MethodToMustUseMessageMethod(&self) {}

    #[must_use]
    pub fn MustUseMethodToMethod(&self) {}

    #[must_use]
    pub fn MustUseMethodToMustUseMessageMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMustUseMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMustUseMessageMethod(&self) {}
}


pub union UnionToStructWithMustUseMethods {
    bar: usize,
}

impl UnionToStructWithMustUseMethods {

    pub fn MethodToMustUseMethod(&self) {}

    pub fn MethodToMustUseMessageMethod(&self) {}

    #[must_use]
    pub fn MustUseMethodToMethod(&self) {}

    #[must_use]
    pub fn MustUseMethodToMustUseMessageMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMustUseMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MustUseMessageMethodToMustUseMessageMethod(&self) {}
}
