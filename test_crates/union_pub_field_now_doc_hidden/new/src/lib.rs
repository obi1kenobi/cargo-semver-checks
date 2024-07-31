// shouldn't flag `union_pub_field_now_doc_hidden` rule
// and flag `union_now_doc_hidden` instead
#[doc(hidden)]
pub union MyUnion {
    f1: u32,
    f2: f32,
}

pub union PublicUnionDocHiddenField {
    
    #[doc(hidden)]
    pub my_field: i8,
}

// shouldn't flag, this is just documented with the string "hidden",
// it's not actually #[doc(hidden)]
pub union PublicUnionFieldDocumentedWithStringHidden {
    #[doc = "hidden"]
    pub f1: u32,
    f2: f32,
}

/// Both the union and its field here will become `#[doc(hidden)]`.
///
/// This is a rare case where we want to report a lint for both the union and the field.
/// Doc-hiddenness on the union means we can't legally *name* it (i.e. import & use it).
/// But if an existing public API returns this union, its pub fields can still be public API
/// without naming the union's type.
#[doc(hidden)]
pub union PublicUnionBothFieldAndUnionDocHidden {
    #[doc(hidden)]
    pub f1: u32,
    f2: f32,
}

// shouldn't flag
pub union UnionWithPrivateField {
    #[doc(hidden)]
    f1: u32,
}
