#![no_std]

#[non_exhaustive]
pub struct PlainStruct {
    pub x: i64,
}

#[non_exhaustive]
pub struct EmptyStruct {}

pub mod my_module {
    #[non_exhaustive]
    pub struct NestedStruct {
        pub x: i64,
    }
}

// Gains two new fields in the new version, so both should be reported separately.
#[non_exhaustive]
pub struct StructGainsTwoFields {
    pub x: i64,
}

// Not #[non_exhaustive], so constructible_struct_adds_field covers this instead.
pub struct ExhaustiveStruct {
    pub x: i64,
}

// Not public, so not part of the public API.
#[non_exhaustive]
struct PrivateStruct {
    pub x: i64,
}

// pub(crate) is not part of the public API.
#[non_exhaustive]
pub(crate) struct CrateStruct {
    pub x: i64,
}

// New field will be private, not pub.
#[non_exhaustive]
pub struct StructGainsPrivateField {
    pub x: i64,
}

// Already has both fields so nothing would change.
#[non_exhaustive]
pub struct UnchangedStruct {
    pub x: i64,
    pub y: i64,
}

// Not #[non_exhaustive] and stays that way, therefore this lint wont fire here.
pub struct WasNeverNonExhaustive {
    pub x: i64,
}
