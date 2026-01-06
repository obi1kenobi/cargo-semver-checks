#![no_std]

// Existing struct shouldn't trigger the lint
pub struct ExistingStructWithHiddenField {
    pub bar: u64,
    #[doc(hidden)]
    pub foo: u64,
}

// New struct with private fields should not trigger this lint
pub struct NewStructWithPrivateField {
    pub bar: u64,
    foo: u64,
}

// This struct is not public, so it should not trigger the lint.
struct PrivateStruct {
    foo: u64,
}

// This struct is #[non_exhaustive], so it should not trigger this lint
#[non_exhaustive]
pub struct NonExhaustiveStructWithPrivateField {
    pub bar: u64,
    foo: u64,
}

// This struct shouldn't trigger the lint because it has public fields only
pub struct StructWithPublicField {
    pub foo: i64,
    pub bar: i64,
}

// This struct should trigget this lint
pub struct StructWithHiddenField {
    pub a: i64,

    #[doc(hidden)]
    pub b: i64,
}
