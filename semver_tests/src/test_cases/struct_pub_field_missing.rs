pub struct FieldWillBeRemoved {
    pub foo: usize,

    /// Testing: <https://doc.rust-lang.org/cargo/reference/semver.html#item-remove>
    #[cfg(not(feature = "struct_pub_field_missing"))]
    pub bar: usize,
}

/// This struct should not be reported by the `struct_pub_field_missing` rule:
/// it will be removed altogether, so the correct rule to catch it is
/// the `struct_missing` rule and not the rule for missing fields.
#[cfg(not(feature = "struct_pub_field_missing"))]
pub struct StructRemoved {
    pub foo: usize,
}
