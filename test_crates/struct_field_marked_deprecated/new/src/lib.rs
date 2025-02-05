/// Normal case, should trigger the lint
pub struct Plain {
    #[deprecated]
    pub field_becomes_deprecated: i64,
    #[deprecated = "This field will be removed in the next major version"]
    pub field_with_deprecation_message: String,
}

pub struct Tuple(
    #[deprecated] pub i64,
    #[deprecated = "Use new_tuple_field instead"] pub String,
);

/// Both the struct and its field here will become `#[deprecated]`.
/// This is a case where we want to report a lint for both the struct and the field.
#[deprecated]
pub struct BothBecomeDeprecated {
    #[deprecated]
    pub field_and_struct_deprecated: i64,
    #[deprecated = "Use new_api_field instead"]
    pub field_with_message_struct_deprecated: String,
}

// Private struct fields becoming deprecated should not trigger the lint
pub struct PrivateFields {
    #[deprecated]
    private_field_becomes_deprecated: i64,
    #[deprecated]
    private_field_also_deprecated: String,
}

// Already deprecated fields should not trigger the lint
pub struct AlreadyDeprecated {
    #[deprecated]
    pub already_deprecated_field: i64,
    pub normal_untouched_field: String,
}

// Private structs should not trigger the lint
struct PrivateStruct {
    #[deprecated]
    pub private_struct_field: i64,
}

// Only the struct becomes deprecated, fields remain normal
#[deprecated = "Use NewStruct instead"]
pub struct StructBecomesDeprecated {
    pub field_stays_public: i64,
    pub field_remains_undeprecated: String,
}
