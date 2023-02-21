// These unions did not have the #[deprecated] attribute in the old version.
// Addition of the attribute should be reported.

#[deprecated]
pub union UnionToDeprecatedUnion {
    bar: u64,
}

#[deprecated = "This attribute was added"]
pub union UnionToDeprecatedMessageUnion {
    bar: u64,
}

// These structs had the #[deprecated] attribute in the old version.
// They also included the user-defined warning message. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

pub union DeprecatedUnionToUnion {
    bar: u64,
}

#[deprecated = "This message was added"]
pub union DeprecatedUnionToDeprecatedMessageUnion {
    bar: u64,
}

pub union DeprecatedMessageUnionToUnion {
    bar: u64,
}

#[deprecated]
pub union DeprecatedMessageUnionToDeprecatedUnion {
    bar: u64,
}

#[deprecated = "This message was changed"]
pub union DeprecatedMessageUnionToDeprecatedMessageUnion {
    bar: u64,
}

// This union is private and should NOT be reported.

#[deprecated]
union DeprecatedPrivateUnion {
    bar: u64,
}

// This union was added in the new version of the crate with its attribute.
// It should NOT be reported by this rule to avoid duplicate lints.
// It should be reported as a new pub type that is part of the crate's API.
// This might seem like a nonsensical test but is a valid edge case.

#[deprecated]
pub union DeprecatedNewUnion {
    bar: u64,
}
