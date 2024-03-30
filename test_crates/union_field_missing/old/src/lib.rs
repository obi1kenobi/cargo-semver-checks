// private union shouldn't cause any breaking changes
union PrivateUnion {
    f1: u32,
    f2: f32,
}


// pub union with private fields shouldn't cause any breaking changes
pub union PubUnionPrivateField {
    f1: u32,
    f2: f32,
}


// pub union with pub fields renamed should cause breaking changes
pub union PubUnionPubFieldRenamed{
    pub f1: u32,
    pub f2: f32,
}

// pub union with pub fields renamed should cause breaking changes
pub union PubUnionPubFieldRemoved{
    pub f1: u32,
    pub f2: f32,
}
