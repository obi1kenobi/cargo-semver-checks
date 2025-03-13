#![no_std]

// Existing Union should not trigger the lint.
pub union ExistingUnion {
    f1: i32,
}

// This union is not public, so it should not be reported by the lint,
union PrivateUnion {
    f1: i32,
}

// Should trigger the lint - new pub union
pub union PubUnion {
    f1: i32,
}
