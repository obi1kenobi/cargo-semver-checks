#![no_std]

// This struct was marked #[non_exhaustive] and has no private fields. It should trigger this lint.
pub struct MyStruct {
    pub field: i32,
}

// Changes in a private struct should not be reported.
struct PrivateStruct {
    pub field: i32,
}

// This struct was not marked #[non_exhaustive] previously.
pub struct AlwaysExhaustiveStruct {
    pub field: i32,
}

// This struct is still non_exhaustive.
#[non_exhaustive]
pub struct NeverExhaustiveStruct {
    pub field: i32,
}

// Changes in a struct with private fields should not be reported.
pub struct StructWithPrivateField {
    pub pub_field: i32,
    priv_field: i32,
}

// The struct gained a private field while losing #[non_exhaustive] (should not be reported).
pub struct GainedPrivateField {
    pub pub_field: i32,
    new_private_field: i32,
}
