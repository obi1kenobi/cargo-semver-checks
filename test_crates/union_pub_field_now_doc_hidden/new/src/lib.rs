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


#[doc(hidden)]
pub union PublicUnionBothFieldAndUnionDocHidden {
    #[doc(hidden)]
    pub f1: u32,
    f2: f32,
}
