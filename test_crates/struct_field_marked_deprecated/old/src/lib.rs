/// Normal case, should trigger the lint
pub struct Plain {
    pub field_becomes_deprecated: i64,
    pub field_with_deprecation_message: String,
}

pub struct Tuple(pub i64, pub String);

/// Both the struct and its field here will become `#[deprecated]`.
/// This is a case where we want to report a lint for both the struct and the field.
pub struct BothBecomeDeprecated {
    pub field_and_struct_deprecated: i64,
    pub field_with_message_struct_deprecated: String,
}

// This struct will become deprecated, but its fields won't
pub struct StructBecomesDeprecated {
    pub field_stays_public: i64,
    pub field_remains_undeprecated: String,
}

// Test doc(hidden) interactions with deprecation
pub struct DocHiddenInteractions {
    // Will become deprecated while already doc(hidden)
    #[doc(hidden)]
    pub already_hidden_field: i32,

    // Will become both doc(hidden) and deprecated
    pub field_becomes_both: String,

    // Will become deprecated only
    pub field_only_deprecated: bool,

    // Already doc(hidden), will stay that way
    #[doc(hidden)]
    pub stays_just_hidden: u64,

    // Already deprecated, will become doc(hidden)
    #[deprecated]
    pub deprecated_becomes_hidden: i32,

    // Already deprecated, stays that way
    #[deprecated]
    pub stays_deprecated: String,

    // Control field - no changes
    pub untouched_field: bool,
}

// Test when struct is doc(hidden)
#[doc(hidden)]
pub struct DocHiddenStruct {
    pub field_becomes_deprecated: i32,
    pub another_field_deprecated: String,
    pub untouched_field: bool,
}

// Private fields should not trigger regardless of attributes
pub struct PrivateFieldCases {
    #[doc(hidden)]
    hidden_becomes_deprecated: i32,

    becomes_both_but_private: String,

    normal_becomes_deprecated: bool,
}

// Private struct - should not trigger regardless of field changes
struct PrivateStructCase {
    #[doc(hidden)]
    pub hidden_field_becomes_deprecated: i32,

    pub becomes_both_attrs: String,
}
