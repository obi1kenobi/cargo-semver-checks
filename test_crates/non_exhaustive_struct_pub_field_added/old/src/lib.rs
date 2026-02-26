#![no_std]

// Adding a new pub field to this struct will trigger this lint
#[non_exhaustive]
pub struct NonExhaustiveStructWithNewPubField {
    pub a: u32,
}

// Adding a private field shouldn't trigger this lint
#[non_exhaustive]
pub struct NonExhaustiveStructWithNewPrivateField {
    pub a: u32,
}

// Adding a doc hidden field shouldn't trigger this lint as it is not public_api_eligible
#[non_exhaustive]
pub struct NonExhaustiveStructWithNewHiddenField {
    pub a: u32,
}

// This struct is not public, so it should not trigger the lint.
#[non_exhaustive]
struct PrivateNonExhaustiveStructWithNewPubField {
    pub a: u32,
}
