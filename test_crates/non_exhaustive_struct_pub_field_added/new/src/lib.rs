#![no_std]

// Adding a new pub field to this struct will trigger this lint
#[non_exhaustive]
pub struct NonExhaustiveStructWithNewPubField {
    pub a: u32,
    pub b: u32, // should be reported
}

// Adding a private field shouldn't trigger this lint
#[non_exhaustive]
pub struct NonExhaustiveStructWithNewPrivateField {
    pub a: u32,
    b: u32, // private, should not be reported
}

// Adding a doc hidden field shouldn't trigger this lint as it is not public_api_eligible
#[non_exhaustive]
pub struct NonExhaustiveStructWithNewHiddenField {
    pub a: u32,
    #[doc(hidden)]
    pub b: u32, // doc-hidden, should not be reported
}

// Adding a new pub field to a private struct shouldn't trigger this lint
#[non_exhaustive]
struct PrivateNonExhaustiveStructWithNewPubField {
    pub a: u32,
    pub b: u32, // private struct, should not be reported
}
