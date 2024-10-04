pub enum PubStructChangedEnum {
    Foo,
}

pub union PubStructChangedUnion {
    foo: usize
}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
/// This is `struct_missing` instead
pub type PubStructChangedType = u8;

/// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
/// This is `constructible_struct_changed_type` instead
pub enum PubStructChangedNoFieldsEnum {}

/// This struct should not be reported by the `struct_pub_field_missing` rule:
/// since the struct is not pub in the first place, changing it does not change the API
enum NonPubStructChangedEnum {
    Foo,
}

/// This struct should not be reported by the `struct_pub_field_missing` rule:
/// since the struct is not pub in the first place, changing it does not change the API
union NonPubStructChangedUnion {
    foo: usize
}

type NonPubStructChangedType = u8;

mod not_pub_visible {
    /// This struct should not be reported by the `struct_pub_field_missing` rule:
    /// since the struct is not in a pub module, changing it does not change the API
    pub enum NonReachabgeStructChangedEnum {
        Foo,
    }
    /// This struct should not be reported by the `struct_pub_field_missing` rule:
    /// since the struct is not in a pub module, changing it does not change the API
    pub union NonReachabgeStructChangedUnion {
        foo: usize
    }
    /// This struct should not be reported by the `struct_pub_field_missing` rule:
    /// since the struct is not in a pub module, changing it does not change the API
    pub type NonReachabgeStructChangedType = u8;
}

