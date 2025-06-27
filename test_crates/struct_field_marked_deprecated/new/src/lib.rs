#![no_std]

/// Normal case, should trigger the lint
pub struct Plain {
    #[deprecated]
    pub field_becomes_deprecated: i64,
    #[deprecated = "This field will be removed in the next major version"]
    pub field_with_deprecation_message: &'static str,
}

pub struct Tuple(
    #[deprecated] pub i64,
    #[deprecated = "Use new_tuple_field instead"] pub &'static str,
);

/// Both the struct and its field here will become `#[deprecated]`.
/// This is a case where we want to report a lint for both the struct and the field.
#[deprecated]
pub struct BothBecomeDeprecated {
    #[deprecated]
    pub field_and_struct_deprecated: i64,
    #[deprecated = "Use new_api_field instead"]
    pub field_with_message_struct_deprecated: &'static str,
}

// Only the struct becomes deprecated, fields remain normal
#[deprecated = "Use NewStruct instead"]
pub struct StructBecomesDeprecated {
    pub field_stays_public: i64,
    pub field_remains_undeprecated: &'static str,
}

// Test doc(hidden) interactions with deprecation
pub struct DocHiddenInteractions {
    // Was already doc(hidden), now also deprecated - should not trigger the lint
    #[doc(hidden)]
    #[deprecated = "Use new_api instead"]
    pub already_hidden_field: i32,

    // Gets both attributes at once
    #[doc(hidden)]
    #[deprecated]
    pub field_becomes_both: &'static str,

    // Only gets deprecated - should trigger the lint
    #[deprecated = "This field is deprecated"]
    pub field_only_deprecated: bool,

    // Stays just doc(hidden) - should not trigger the lint
    #[doc(hidden)]
    pub stays_just_hidden: u64,

    // Was deprecated, now also hidden - should not trigger the lint
    #[deprecated]
    #[doc(hidden)]
    pub deprecated_becomes_hidden: i32,

    // No change - stays deprecated - should not trigger the lint
    #[deprecated]
    pub stays_deprecated: &'static str,

    // Control field - no changes
    pub untouched_field: bool,
}

// Test when struct is doc(hidden) - should not trigger the lint
#[doc(hidden)]
pub struct DocHiddenStruct {
    #[deprecated]
    pub field_becomes_deprecated: i32,
    #[deprecated = "Use new_api instead"]
    pub another_field_deprecated: &'static str,
    pub untouched_field: bool,
}

// Private fields should not trigger regardless of attributes
pub struct PrivateFieldCases {
    #[doc(hidden)]
    #[deprecated]
    hidden_becomes_deprecated: i32,

    #[doc(hidden)]
    #[deprecated]
    becomes_both_but_private: &'static str,

    #[deprecated]
    normal_becomes_deprecated: bool,
}

// Private struct - should not trigger regardless of field changes
struct PrivateStructCase {
    #[doc(hidden)]
    #[deprecated]
    pub hidden_field_becomes_deprecated: i32,

    #[doc(hidden)]
    #[deprecated]
    pub becomes_both_attrs: &'static str,
}
