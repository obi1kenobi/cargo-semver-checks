pub union MyUnion {
    f1: u32,
    f2: f32,
}

pub union PublicUnionDocHiddenField {
    pub my_field: i8,
}

pub union PublicUnionFieldDocumentedWithStringHidden {
    pub f1: u32,
    f2: f32,
}

pub union PublicUnionBothFieldAndUnionDocHidden {
    pub f1: u32,
    f2: f32,
}

pub union UnionWithPrivateField {
    f1: u32,
}
