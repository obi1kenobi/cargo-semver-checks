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

// Private struct fields becoming deprecated should not trigger the lint
pub struct PrivateFields {
    private_field_becomes_deprecated: i64,
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
    pub private_struct_field: i64,
}

// This struct will become deprecated, but its fields won't
pub struct StructBecomesDeprecated {
    pub field_stays_public: i64,
    pub field_remains_undeprecated: String,
}
