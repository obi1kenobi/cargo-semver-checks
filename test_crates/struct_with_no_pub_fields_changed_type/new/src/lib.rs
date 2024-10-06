pub enum PubStructChangedToEnum {
    Foo(usize),
}

pub union PubStructChangedToUnion {
    foo: usize
}

#[non_exhaustive]
pub enum PubNonExhaustiveStructChangedToEnum {
    Foo(usize),
}

pub union PubNonExhaustiveStructChangedToUnion {
    foo: usize,
}

pub enum PubStructWithNonPubDocFieldChangedToEnum {
    Foo(usize),
}

pub union PubStructWithNonPubDocFieldChangedToUnion {
    /// Despite this field being pub, hiding it makes this not be `public_api_eligible` anymore
    /// This struct should trigger `struct_with_no_pub_fields_changed_type` instead of
    /// `struct_with_pub_fields_changed_type`
    #[doc(hidden)]
    pub foo: usize,
}

pub enum PubStructWithNonPubDocFieldAndNonPubFieldChangedToEnum {
    Foo(usize),
    Bar(usize),
}

pub union PubStructWithNonPubDocFieldAndNonPubFieldChangedToUnion {
    foo: usize,
    /// Despite this field being pub, hiding it makes this not be `public_api_eligible` anymore
    /// This struct should trigger `struct_with_no_pub_fields_changed_type` instead of
    /// `struct_with_pub_fields_changed_type`
    #[doc(hidden)]
    pub bar: usize,
}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed_type` rule:
/// The struct is not `public_api_eligible`.
/// See <https://predr.ag/blog/checking-semver-for-doc-hidden-items/> for additional context
#[doc(hidden)]
pub enum NonPubDocStructChangedToEnum {
    Foo(usize),
}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed_type` rule:
/// The struct is not `public_api_eligible`.
/// See <https://predr.ag/blog/checking-semver-for-doc-hidden-items/> for additional context
#[doc(hidden)]
pub union NonPubDocStructChangedToUnion {
    foo: usize,
}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed_type` rule:
/// This is `struct_with_pub_fields_changed_type` instead
pub enum PubStructWithPubFieldChangedToEnum {
    Foo(usize),
    Bar(usize),
}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed_type` rule:
/// This is `struct_with_pub_fields_changed_type` instead
pub union PubStructWithPubFieldChangedToUnion {
    foo: usize,
    pub bar: usize,
}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed_type` rule:
/// This is `struct_missing` instead
pub type PubStructChangedToType = u8;

/// This struct should not be reported by the `struct_with_no_pub_fields_changed_type` rule:
/// This is `constructible_struct_changed_type` instead
pub enum PubStructChangedToNoFieldsEnum {}

/// This struct should not be reported by the `struct_pub_field_missing` rule:
/// since the struct is not pub in the first place, changing it does not change the API
enum NonPubStructChangedToEnum {
    Foo(usize),
}

/// This struct should not be reported by the `struct_pub_field_missing` rule:
/// since the struct is not pub in the first place, changing it does not change the API
union NonPubStructChangedToUnion {
    foo: usize
}

type NonPubStructChangedToType = u8;

mod not_pub_visible {
    /// This struct should not be reported by the `struct_pub_field_missing` rule:
    /// since the struct is not in a pub module, changing it does not change the API
    pub enum NonReachableStructChangedToEnum {
        Foo(usize),
    }

    /// This struct should not be reported by the `struct_pub_field_missing` rule:
    /// since the struct is not in a pub module, changing it does not change the API
    pub union NonReachableStructChangedToUnion {
        foo: usize
    }

    /// This struct should not be reported by the `struct_pub_field_missing` rule:
    /// since the struct is not in a pub module, changing it does not change the API
    pub type NonReachableStructChangedToType = u8;
}
