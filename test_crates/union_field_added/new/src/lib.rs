#![no_std]

// Should trigger `union_field_added_with_all_pub_fields`.
#[repr(C)]
pub union AllPubFields {
    pub a: [i32; 2],
    pub b: i64,
    pub c: *const i64,
}

// Should trigger `union_field_added_with_non_pub_fields`.
#[repr(C)]
pub union SomeHiddenPubFields {
    pub a: [i32; 2],

    #[doc(hidden)]
    pub b: i64,

    #[doc(hidden)]
    pub c: *const i64,
}

// Should trigger `union_field_added_with_non_pub_fields`.
#[repr(C)]
pub union SomePrivateFields {
    pub a: [i32; 2],
    pub(crate) b: i64,
    c: *const i64,
}

// Shouldn't trigger the "union field added" lints,
// but will trigger the "repr(C) removed" lint for unions.
pub union ReprCRemovedAllPublicFields {
    pub a: [i32; 2],
    pub b: i64,
    pub c: *const i64,
}

// Shouldn't trigger the "union field added" lints,
// but will trigger the "repr(C) removed" lint for unions.
pub union ReprCRemovedNonPublicFields {
    pub a: [i32; 2],
    b: i64,
    c: *const i64,
}

// Shouldn't trigger any of the lints.
#[repr(C)]
pub union BecameReprC {
    pub a: [i32; 2],
    pub b: i64,
    pub c: *const i64,
}

// Should trigger `union_field_added_with_all_pub_fields`
// even though a field also became non-public-API.
#[repr(C)]
pub union FieldBecameNonPublic {
    pub a: [i32; 2],
    b: i64,
    c: *const i64,
}

// Should trigger `union_field_added_with_all_pub_fields`
// even though a field also became non-public-API.
#[repr(C)]
pub union FieldBecameNonPublicAPI {
    pub a: [i32; 2],

    #[doc(hidden)]
    pub b: i64,

    pub c: *const i64,
}

// Should trigger `union_field_added_with_non_pub_fields`
// even though the non-public-API field also became public API.
#[repr(C)]
pub union HiddenFieldBecamePublicAPI {
    pub a: [i32; 2],
    pub b: i64,
    pub c: *const i64,
}

// Should trigger `union_field_added_with_non_pub_fields`
// even though the non-public field also became public API.
#[repr(C)]
pub union PrivateFieldBecamePublicAPI {
    pub a: [i32; 2],
    pub b: i64,
    pub c: *const i64,
}
