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

// This struct has never been #[non_exhaustive] and should not be reported.
pub struct WasNeverNonExhaustive {
    pub x: i64,
}

// Was #[non_exhaustive] in the baseline; the new pub field should be reported.
// struct_no_longer_non_exhaustive fires separately for losing the attribute.
#[non_exhaustive]
pub struct LosesNonExhaustiveAndGainsField {
    pub x: i64,
}

// The field y is private here and becomes public in the new version.
// That's a separate change and should not be reported by this lint.
#[non_exhaustive]
pub struct PrivateFieldBecomesPublic {
    pub x: i64,
    y: i64,
}
