// These unions did not have the #[must_use] attribute in the old version.
// Addition of the attribute should be reported by this rule.

pub union UnionToMustUseUnion {
    bar: u64,
}

pub union UnionToMustUseMessageUnion {
    bar: u64,
}


// These unions had the #[must_use] attribute in the old version. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

#[must_use]
pub union MustUseUnionToUnion {
    bar: u64,
}

#[must_use]
pub union MustUseUnionToMustUseMessageUnion {
    bar: u64,
}


// These unions had the #[must_use] attribute in the old version.
// They also included the user-defined warning message. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

#[must_use = "Foo"]
pub union MustUseMessageUnionToUnion {
    bar: u64,
}

#[must_use = "Foo"]
pub union MustUseMessageUnionToMustUseUnion {
    bar: u64,
}

#[must_use = "Foo"]
pub union MustUseMessageUnionToMustUseMessageUnion {
    bar: u64,
}


// This union is private and should NOT be reported by this rule.

union MustUsePrivateUnion {
    bar: u64,
}
