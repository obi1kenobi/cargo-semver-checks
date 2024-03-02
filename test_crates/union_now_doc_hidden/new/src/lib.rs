#[doc(hidden)]
pub union MyUnion {
    f1: u32,
    f2: f32,
}

pub union PublicUnionHiddenField {
    // shouldn't flag `Union_now_doc_hidden` rule
    // as this is a field that's hidden,
    // not the entire Union
    #[doc(hidden)]
    pub my_field: i8,
}

#[doc = "hidden"] // shouldn't flag, this is just documented with the string "hidden",
                  // it's not actually #[doc(hidden)]
pub union PublicUnionDocumentedWithStringHidden {
    f1: u32,
    f2: f32,
}
