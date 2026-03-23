#![no_std]

// These unions did not have the #[deprecated] attribute in the old version.
// Addition of the attribute should be reported by this rule.

#[deprecated]
pub union UnionToDeprecatedUnion {
    pub bar: u64,
}

#[deprecated = "This union is deprecated"]
pub union UnionToDeprecatedMessageUnion {
    pub bar: u64,
}


// These unions already had the #[deprecated] attribute in the old version.
// Changes to the attribute, including deletion, should NOT be reported by this rule.

pub union DeprecatedUnionToUnion {
    pub bar: u64,
}

#[deprecated = "new message"]
pub union DeprecatedMessageUnionToDeprecatedMessageUnion {
    pub bar: u64,
}


// This union is private and should NOT be reported by this rule.

#[deprecated]
union PrivateUnion {
    bar: u64,
}


// This union is #[doc(hidden)] and should NOT be reported by this rule.

#[deprecated]
#[doc(hidden)]
pub union DocHiddenUnion {
    pub bar: u64,
}


// This union was added in the new version of the crate with the attribute.
// It should NOT be reported by this rule to avoid duplicate lints.

#[deprecated]
pub union DeprecatedNewUnion {
    pub bar: u64,
}
