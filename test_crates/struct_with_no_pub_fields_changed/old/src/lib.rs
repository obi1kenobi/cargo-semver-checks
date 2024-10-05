pub struct PubStructChangedToEnum {
    foo: usize,
}

pub struct PubStructChangedToUnion {
    foo: usize,
}

pub struct PubStructWithNonPubDocChangedToEnum {
    foo: usize,
    /// Despite this field being pub, hiding it makes this not be `public_api_eligible` anymore
    /// This struct should trigger `struct_with_no_pub_fields_changed` instead of `struct_with_pub_fields_changed`
    #[doc(hidden)]
    pub bar: usize,
}


pub struct PubStructWithNonPubDocChangedToUnion {
    foo: usize,
    /// Despite this field being pub, hiding it makes this not be `public_api_eligible` anymore
    /// This struct should trigger `struct_with_no_pub_fields_changed` instead of `struct_with_pub_fields_changed`
    #[doc(hidden)]
    pub bar: usize,
}



/// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
/// The struct is not `public_api_eligible`.
/// See https://predr.ag/blog/checking-semver-for-doc-hidden-items/ for additional context
#[doc(hidden)]
pub struct NonPubDocStructChangedToEnum {
    foo: usize,
}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
/// The struct is not `public_api_eligible`.
/// See https://predr.ag/blog/checking-semver-for-doc-hidden-items/ for additional context
#[doc(hidden)]
pub struct NonPubDocStructChangedToUnion {
    foo: usize,
}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
/// This is `struct_with_pub_fields_changed` instead
pub struct PubStructWithPubFieldChangedToEnum {
    foo: usize,
    pub bar: usize,
}


/// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
/// This is `struct_with_pub_fields_changed` instead
pub struct PubStructWithPubFieldChangedToUnion {
    foo: usize,
    pub bar: usize,

}


/// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
/// This is `struct_missing` instead
pub struct PubStructChangedToType(u8);

/// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
/// This is `constructible_struct_changed_type` instead
pub struct PubStructChangedToNoFieldsEnum {}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
/// since the struct is not pub in the first place, changing it does not change the API
struct NonPubStructChangedToEnum {
    foo: usize,
}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
/// since the struct is not pub in the first place, changing it does not change the API
struct NonPubStructChangedToUnion {
    foo: usize,
}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
/// since the struct is not pub in the first place, changing it does not change the API
struct NonPubStructChangedToType(u8);

mod not_pub_visible {
    /// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
    /// since the struct is not in a pub module, changing it does not change the API
    pub struct NonReachabgeStructChangedToEnum {
        foo: usize,
    }
    /// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
    /// since the struct is not in a pub module, changing it does not change the API
    pub struct NonReachabgeStructChangedToUnion {
        foo: usize,
    }
    /// This struct should not be reported by the `struct_pub_field_missing` rule:
    /// since the struct is not in a pub module, changing it does not change the API
    pub struct NonReachabgeStructChangedToType(u8);
}

