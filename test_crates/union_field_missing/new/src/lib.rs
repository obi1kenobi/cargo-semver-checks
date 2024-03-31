// private union shouldn't cause any breaking changes
union PrivateUnion {
    f3: f32,
}


// pub union with private fields shouldn't cause any breaking changes
pub union PubUnionPrivateField {
    f3: f32,
}


// pub union with pub fields renamed should cause breaking changes
pub union PubUnionPubFieldRenamed{
    pub f1: u32,
    pub f3: f32,
}

// pub union with pub fields removed should cause breaking changes
pub union PubUnionPubFieldRemoved{
    pub f1: u32,
}
