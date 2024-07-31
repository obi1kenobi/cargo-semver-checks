// These unions did not have the #[must_use] attribute in the old version.
// Addition of the attribute should be reported by this rule.

#[must_use]
pub union UnionToMustUseUnion {
    bar: u64,
}

#[must_use = "Foo"]
pub union UnionToMustUseMessageUnion {
    bar: u64,
}


// These unions had the #[must_use] attribute in the old version. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

pub union MustUseUnionToUnion {
    bar: u64,
}

#[must_use = "Foo"]
pub union MustUseUnionToMustUseMessageUnion {
    bar: u64,
}


// These unions had the #[must_use] attribute in the old version.
// They also included the user-defined warning message. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

pub union MustUseMessageUnionToUnion {
    bar: u64,
}

#[must_use]
pub union MustUseMessageUnionToMustUseUnion {
    bar: u64,
}

#[must_use = "Baz"]
pub union MustUseMessageUnionToMustUseMessageUnion {
    bar: u64,
}


// This union is private and should NOT be reported by this rule.

#[must_use]
union MustUsePrivateUnion {
    bar: u64,
}


// This union was added in the new version of the crate with its attribute.
// It should NOT be reported by this rule to avoid duplicate lints.
// It should be reported as a new pub type that is part of the crate's API.

#[must_use]
pub union MustUseNewUnion {
    bar: u64,
}
