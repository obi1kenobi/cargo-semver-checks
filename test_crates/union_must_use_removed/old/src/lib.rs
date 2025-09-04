#![no_std]

// These unions did not have the #[must_use] attribute in the old version.
// Addition of the attribute should NOT be reported by this rule.

pub union UnionToMustUseUnion {
    bar: u64,
}

pub union UnionToMustUseMessageUnion {
    bar: u64,
}


// These unions had the #[must_use] attribute in the old version. Removal of
// the attribute SHOULD be reported by this rule.

#[must_use]
pub union MustUseUnionToUnion {
    bar: u64,
}

#[must_use = "Foo"]
pub union MustUseMessageUnionToUnion {
    bar: u64,
}



// These unions had the #[must_use] attribute in the old version.
// They sometimes included the user-defined warning message. Changes of
// the attribute, except deletion, should NOT be reported by this rule.

#[must_use]
pub union MustUseUnionToMustUseMessageUnion {
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

#[must_use]
union MustUsePrivateUnion {
    bar: u64,
}

// This union was removed in the new version of the crate with its attribute.
// It should NOT be reported by this rule to avoid duplicate lints.
// It should be reported as a removed pub type that is part of the crate's API.

#[must_use]
pub union MustUseRemovedUnion {
    bar: u64,
}
