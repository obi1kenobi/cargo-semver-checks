union PubUnionBecomesPrivate {
    f1: i32,
}

pub mod public_mod {
    pub union PubUnion {
        f1: i32,
    }
}

// should not trigger union_missing so long as path remains importable
pub struct PubUnionBecomesStruct {
    f1: i32,
}
