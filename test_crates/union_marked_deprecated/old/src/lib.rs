#![no_std]

// These unions did not have the #[deprecated] attribute in the old version.
// Addition of the attribute should be reported by this rule.

pub union UnionToDeprecatedUnion {
    pub bar: u64,
}

pub union UnionToDeprecatedMessageUnion {
    pub bar: u64,
}


// These unions already had the #[deprecated] attribute in the old version.
// Changes to the attribute, including deletion, should NOT be reported by this rule.

#[deprecated]
pub union DeprecatedUnionToUnion {
    pub bar: u64,
}

#[deprecated = "old message"]
pub union DeprecatedMessageUnionToDeprecatedMessageUnion {
    pub bar: u64,
}


// This union is private and should NOT be reported by this rule.

union PrivateUnion {
    bar: u64,
}


// This union is #[doc(hidden)] and should NOT be reported by this rule.

#[doc(hidden)]
pub union DocHiddenUnion {
    pub bar: u64,
}
