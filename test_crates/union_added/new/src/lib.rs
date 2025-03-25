#![no_std]

// Existing Union should not trigger the lint.
pub union ExistingUnion {
    f1: i32,
}

// This is a private union, so it should not be reported by the lint.
union PrivateUnion {
    f1: i32,
}

// This is a new pub union, so it should be reported by the lint.
pub union PubUnion {
    f1: i32,
}
