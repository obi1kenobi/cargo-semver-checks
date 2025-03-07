#![no_std]

// private union shouldn't cause any breaking changes
union PrivateUnion {
    f3: f32,
}

// pub union with private fields shouldn't cause any breaking changes
pub union PubUnionPrivateField {
    f3: f32,
}

// pub union with pub fields renamed should cause breaking changes
pub union PubUnionPubFieldRenamed {
    pub f1: u32,
    pub f3: f32,
}

// pub union with pub fields removed should cause breaking changes
pub union PubUnionPubFieldRemoved {
    pub f1: u32,
}

// this should only trigger the union_missing lint, not union_field_missing
union PubUnionBecomesPrivateAndPubFieldRemoved {
    pub f1: u32,
    f2: f32,
}
