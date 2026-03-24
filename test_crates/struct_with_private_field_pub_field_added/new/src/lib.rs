#![no_std]

// Gained a new pub field z.
pub struct PlainStruct {
    pub x: i64,
    pub z: i64,
    y: i64,
}

pub mod my_module {
    // Gained a new pub field z.
    pub struct NestedStruct {
        pub x: i64,
        pub z: i64,
        y: i64,
    }
}

// Gained two new pub fields y2 and z.
pub struct StructGainsTwoFields {
    pub x: i64,
    pub y2: i64,
    pub z: i64,
    y: i64,
}

// Gained a new pub field y therefore the struct had no pub fields in the baseline.
pub struct AllPrivateFields {
    pub y: i64,
    x: i64,
}

// No private fields, so constructible_struct_adds_field covers this instead.
pub struct ExhaustiveAllPublicStruct {
    pub x: i64,
    pub z: i64,
}

// Has #[non_exhaustive], so non_exhaustive_struct_pub_field_added handles the new field.
#[non_exhaustive]
pub struct NonExhaustiveStruct {
    pub x: i64,
    pub z: i64,
    y: i64,
}

// Not public, so not part of the public API.
struct PrivateStruct {
    pub x: i64,
    pub z: i64,
    y: i64,
}

// pub(crate) is not part of the public API.
pub(crate) struct CrateStruct {
    pub x: i64,
    pub z: i64,
    y: i64,
}

// Nothing changed.
pub struct UnchangedStruct {
    pub x: i64,
    pub y: i64,
    z: i64,
}

// New field is private, not pub.
pub struct StructGainsPrivateField {
    pub x: i64,
    y: i64,
    z: i64,
}

// The previously private field y is now public, but z remains private.
// Different kind of change so this lint should not fire for it.
pub struct PrivateFieldBecomesPublic {
    pub x: i64,
    pub y: i64,
    z: i64,
}

// The only private field y is now public; the new pub field z should be reported.
pub struct PrivateBecomesPubGainsField {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

// The only private field y was removed; the new pub field z should be reported.
pub struct PrivateRemovedGainsField {
    pub x: i64,
    pub z: i64,
}
