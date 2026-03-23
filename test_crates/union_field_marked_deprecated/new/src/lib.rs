#![no_std]

// ---- Positive cases: field should be detected ----

// The `bar` field is now deprecated. Should be reported.
pub union FieldBecomesDeprecated {
    #[deprecated]
    pub bar: u64,
    pub baz: u32,
}

// ---- Negative cases: should NOT be detected ----

// Union itself becomes deprecated — fields inherit deprecation,
// so individual field deprecation should not be double-reported.
#[deprecated]
pub union DeprecatedUnion {
    #[deprecated]
    pub bar: u64,
}

// Field was already deprecated in old version. Not newly deprecated.
pub union FieldAlreadyDeprecated {
    #[deprecated]
    pub bar: u64,
    pub baz: u32,
}

// Private union — should never appear in results.
union PrivateUnion {
    #[deprecated]
    bar: u64,
}

// Doc-hidden union — should not be reported.
#[doc(hidden)]
pub union DocHiddenUnion {
    #[deprecated]
    pub bar: u64,
}
