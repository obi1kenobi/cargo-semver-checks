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

// Not #[non_exhaustive], so constructible_struct_adds_field covers field additions here.
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

// Not #[non_exhaustive] and unchanged from baseline, so not reported.
pub struct WasNeverNonExhaustive {
    pub x: i64,
}

// Was #[non_exhaustive] in the baseline; the new pub field should be reported.
// struct_no_longer_non_exhaustive fires separately for losing the attribute.
pub struct LosesNonExhaustiveAndGainsField {
    pub x: i64,
    pub y: i64,
}

// The field y was private in the baseline and is now public.
// That's a separate change and should not be reported by this lint.
#[non_exhaustive]
pub struct PrivateFieldBecomesPublic {
    pub x: i64,
    pub y: i64,
}
