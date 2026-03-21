#![no_std]

#[non_exhaustive]
pub struct PlainStruct {
    pub x: i64,
    pub y: i64,
}

#[non_exhaustive]
pub struct EmptyStruct {
    pub x: i64,
}

pub mod my_module {
    #[non_exhaustive]
    pub struct NestedStruct {
        pub x: i64,
        pub y: i64,
    }
}

// Gains two new fields, so both should be reported separately.
#[non_exhaustive]
pub struct StructGainsTwoFields {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

// Not #[non_exhaustive], so constructible_struct_adds_field covers this instead.
// Kept unchanged to avoid triggering that lint.
pub struct ExhaustiveStruct {
    pub x: i64,
}

// Not public, so not part of the public API.
#[non_exhaustive]
struct PrivateStruct {
    pub x: i64,
    pub y: i64,
}

// pub(crate) is not part of the public API.
#[non_exhaustive]
pub(crate) struct CrateStruct {
    pub x: i64,
    pub y: i64,
}

// New field is private, not pub.
#[non_exhaustive]
pub struct StructGainsPrivateField {
    pub x: i64,
    y: i64,
}

// Already had both fields in the baseline so nothing changed.
#[non_exhaustive]
pub struct UnchangedStruct {
    pub x: i64,
    pub y: i64,
}

// Was not #[non_exhaustive] in baseline, kept unchanged.
pub struct WasNeverNonExhaustive {
    pub x: i64,
}
