pub struct PubStructChangedEnum {
    foo: usize,
}

pub struct PubStructChangedUnion {
    foo: usize,
}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
/// This is `struct_missing` instead
pub struct PubStructChangedType(u8);

/// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
/// This is `constructible_struct_changed_type` instead
pub struct PubStructChangedNoFieldsEnum {}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
/// since the struct is not pub in the first place, changing it does not change the API
struct NonPubStructChangedEnum {
    foo: usize,
}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
/// since the struct is not pub in the first place, changing it does not change the API
struct NonPubStructChangedUnion {
    foo: usize,
}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
/// since the struct is not pub in the first place, changing it does not change the API
struct NonPubStructChangedType(u8);

mod not_pub_visible {
    /// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
        /// since the struct is not in a pub module, changing it does not change the API
        pub struct NonReachabgeStructChangedEnum {
            foo: usize,
        }
        /// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
        /// since the struct is not in a pub module, changing it does not change the API
        pub struct NonReachabgeStructChangedUnion {
            foo: usize,
        }
        /// This struct should not be reported by the `struct_pub_field_missing` rule:
        /// since the struct is not in a pub module, changing it does not change the API
        pub struct NonReachabgeStructChangedType(u8);
}

