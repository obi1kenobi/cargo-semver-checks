pub enum Enum {
    FieldWillBeMissing {
        foo: usize,

        /// Testing: <https://doc.rust-lang.org/cargo/reference/semver.html#item-remove>
        #[cfg(not(feature = "enum_structvariant_field_missing"))]
        bar: usize,
    }
}

/// This struct variant should not be reported by the `enum_structvariant_field_missing` rule:
/// it will be removed altogether, so the correct rule to catch it is
/// the `enum_variant_missing` rule and not the rule for missing fields.
pub enum IgnoredEnum {
    #[cfg(not(feature = "enum_structvariant_field_missing"))]
    StructVariantWillBeMissing {
        foo: usize,
    }
}
