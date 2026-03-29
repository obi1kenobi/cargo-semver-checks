#![no_std]

// ---- Positive cases: field should be detected ----

// A public union whose public field becomes deprecated.
pub union FieldBecomesDeprecated {
    pub bar: u64,
    pub baz: u32,
}

// ---- Negative cases: should NOT be detected ----

// Union itself is already deprecated in old version — fields inherit deprecation,
// so individual field deprecation should not be double-reported.
#[deprecated]
pub union DeprecatedUnion {
    pub bar: u64,
}

// Field is already deprecated in old version, so any change (including removal)
// should not be reported by this rule.
pub union FieldAlreadyDeprecated {
    #[deprecated]
    pub bar: u64,
    pub baz: u32,
}

// Private union — should never appear in results.
union PrivateUnion {
    bar: u64,
}

// Doc-hidden union — should not be reported.
#[doc(hidden)]
pub union DocHiddenUnion {
    pub bar: u64,
}
