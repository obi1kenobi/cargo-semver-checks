pub enum PubStructChangedToEnum {
    Foo,
}

pub union PubStructChangedToUnion {
    foo: usize
}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
/// This is `struct_with_pub_fields_changed` instead
pub enum PubStructWithPubFieldChangedToEnum {
    Foo(usize),
    Bar(usize),
}


/// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
/// This is `struct_with_pub_fields_changed` instead
pub union PubStructWithPubFieldChangedToUnion {
    foo: usize,
    pub bar: usize,

}

/// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
/// This is `struct_missing` instead
pub type PubStructChangedToType = u8;

/// This struct should not be reported by the `struct_with_no_pub_fields_changed` rule:
/// This is `constructible_struct_changed_type` instead
pub enum PubStructChangedToNoFieldsEnum {}

/// This struct should not be reported by the `struct_pub_field_missing` rule:
/// since the struct is not pub in the first place, changing it does not change the API
enum NonPubStructChangedToEnum {
    Foo,
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
    pub enum NonReachabgeStructChangedToEnum {
        Foo,
    }
    /// This struct should not be reported by the `struct_pub_field_missing` rule:
    /// since the struct is not in a pub module, changing it does not change the API
    pub union NonReachabgeStructChangedToUnion {
        foo: usize
    }
    /// This struct should not be reported by the `struct_pub_field_missing` rule:
    /// since the struct is not in a pub module, changing it does not change the API
    pub type NonReachabgeStructChangedToType = u8;
}

