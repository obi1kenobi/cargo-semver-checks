// Adding #[deprecated] to these unions should be reported.

pub union UnionToDeprecatedUnion {
    bar: u64,
}

pub union UnionToDeprecatedMessageUnion {
    bar: u64,
}

// These unions already have the attribute and changes to the attribute should not be reported.

#[deprecated]
pub union DeprecatedUnionToUnion {
    bar: u64,
}

#[deprecated]
pub union DeprecatedUnionToDeprecatedMessageUnion {
    bar: u64,
}

#[deprecated = "This attribute will be deleted"]
pub union DeprecatedMessageUnionToUnion {
    bar: u64,
}

#[deprecated = "This message will be deleted"]
pub union DeprecatedMessageUnionToDeprecatedUnion {
    bar: u64,
}

#[deprecated = "This message will change"]
pub union DeprecatedMessageUnionToDeprecatedMessageUnion {
    bar: u64,
}

// This union is private and should NOT be reported.

union DeprecatedPrivateUnion {
    bar: u64,
}
