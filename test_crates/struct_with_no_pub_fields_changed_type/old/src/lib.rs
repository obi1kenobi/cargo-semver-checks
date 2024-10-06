pub struct PubStructChangedToEnum {
    foo: usize,
}

pub struct PubStructChangedToUnion {
    foo: usize,
}

#[non_exhaustive]
pub struct PubNonExhaustiveStructChangedToEnum {
    foo: usize,
}

#[non_exhaustive]
pub struct PubNonExhaustiveStructChangedToUnion {
    foo: usize,
}

pub struct PubStructWithNonPubDocFieldChangedToEnum {
    /// Despite this field being pub, hiding it makes this not be `public_api_eligible` anymore
    /// This struct should trigger `struct_with_no_pub_fields_changed_type` instead of
    /// `struct_with_pub_fields_changed_type`
    /// See <https://predr.ag/blog/checking-semver-for-doc-hidden-items/> for additional context
    #[doc(hidden)]
    pub foo: usize,
}

pub struct PubStructWithNonPubDocFieldChangedToUnion {
    /// Despite this field being pub, hiding it makes this not be `public_api_eligible` anymore
    /// This struct should trigger `struct_with_no_pub_fields_changed_type` instead of
    /// `struct_with_pub_fields_changed_type`
    /// See <https://predr.ag/blog/checking-semver-for-doc-hidden-items/> for additional context
    #[doc(hidden)]
    pub foo: usize,
}

pub struct PubStructWithNonPubDocFieldAndNonPubFieldChangedToEnum {
    foo: usize,
    /// Despite this field being pub, hiding it makes this not be `public_api_eligible` anymore
    /// This struct should trigger `struct_with_no_pub_fields_changed_type` instead of
    /// `struct_with_pub_fields_changed_type`
    /// See <https://predr.ag/blog/checking-semver-for-doc-hidden-items/> for additional context
    #[doc(hidden)]
    pub bar: usize,
}

pub struct PubStructWithNonPubDocFieldAndNonPubFieldChangedToUnion {
    foo: usize,
    /// Despite this field being pub, hiding it makes this not be `public_api_eligible` anymore
    /// This struct should trigger `struct_with_no_pub_fields_changed_type` instead of
    /// `struct_with_pub_fields_changed_type`
    /// See <https://predr.ag/blog/checking-semver-for-doc-hidden-items/> for additional context
    #[doc(hidden)]
    pub bar: usize,
}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed_type` rule:
/// The struct is not `public_api_eligible`.
/// See <https://predr.ag/blog/checking-semver-for-doc-hidden-items/> for additional context
#[doc(hidden)]
pub struct NonPubDocStructChangedToEnum {
    foo: usize,
}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed_type` rule:
/// The struct is not `public_api_eligible`.
/// See <https://predr.ag/blog/checking-semver-for-doc-hidden-items/> for additional context
#[doc(hidden)]
pub struct NonPubDocStructChangedToUnion {
    foo: usize,
}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed_type` rule:
/// This is `struct_with_pub_fields_changed_type` instead
pub struct PubStructWithPubFieldChangedToEnum {
    foo: usize,
    pub bar: usize,
}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed_type` rule:
/// This is `struct_with_pub_fields_changed_type` instead
pub struct PubStructWithPubFieldChangedToUnion {
    foo: usize,
    pub bar: usize,
}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed_type` rule:
/// This is `struct_missing` instead
pub struct PubStructChangedToType(u8);

/// This struct should not be reported by the `struct_with_no_pub_fields_changed_type` rule:
/// This is `constructible_struct_changed_type` instead
pub struct PubStructChangedToNoFieldsEnum {}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed_type` rule:
/// since the struct is not pub in the first place, changing it does not change the API
struct NonPubStructChangedToEnum {
    foo: usize,
}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed_type` rule:
/// since the struct is not pub in the first place, changing it does not change the API
struct NonPubStructChangedToUnion {
    foo: usize,
}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed_type` rule:
/// since the struct is not pub in the first place, changing it does not change the API
struct NonPubStructChangedToType(u8);

mod not_pub_visible {
    /// This struct should not be reported by the `struct_with_no_pub_fields_changed_type` rule:
    /// since the struct is not in a pub module, changing it does not change the API
    pub struct NonReachableStructChangedToEnum {
        foo: usize,
    }

    /// This struct should not be reported by the `struct_with_no_pub_fields_changed_type` rule:
    /// since the struct is not in a pub module, changing it does not change the API
    pub struct NonReachableStructChangedToUnion {
        foo: usize,
    }

    /// This struct should not be reported by the `struct_pub_field_missing` rule:
    /// since the struct is not in a pub module, changing it does not change the API
    pub struct NonReachableStructChangedToType(u8);
}
