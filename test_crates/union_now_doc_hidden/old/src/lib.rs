pub union MyUnion {
    f1: u32,
    f2: f32,
}

pub union PublicUnionHiddenField {
    pub my_field: i8,
}

pub union PublicUnionDocumentedWithStringHidden {
    f1: u32,
    f2: f32,
}
