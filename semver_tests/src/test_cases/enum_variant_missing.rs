pub enum VariantWillBeRemoved {
    Foo,

    /// Testing: <https://doc.rust-lang.org/cargo/reference/semver.html#item-remove>
    #[cfg(not(feature = "enum_variant_missing"))]
    Bar,
}

/// This enum should not be reported by the `enum_variant_missing` rule:
/// it will be removed altogether, so the correct rule to catch it is
/// the `enum_missing` rule and not the rule for missing variants.
#[cfg(not(feature = "enum_variant_missing"))]
pub enum ShouldNotMatch {}
