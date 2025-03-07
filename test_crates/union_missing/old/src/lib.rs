#![no_std]

pub union PubUnionRemoved {
    f1: i32,
}

pub union PubUnionBecomesPrivate {
    f1: i32,
}

pub mod public_mod {
    pub union PubUnion {
        f1: i32,
    }
}

pub use public_mod::PubUnion as PubUseUnionRemoved;

// should not trigger union_missing so long as path remains importable
pub union PubUnionBecomesStruct {
    f1: i32,
}

// the following unions should not trigger lints when removed
union PrivateUnion {
    f1: i32,
}

mod private_mod {
    pub union PrivatePathPubUnion {
        f1: i32,
    }
}

#[doc(hidden)]
pub union DocHiddenUnion {
    f1: i32,
}
