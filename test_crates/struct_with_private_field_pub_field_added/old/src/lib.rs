#![no_std]

// Has one private field, gains a new pub field in the new version.
pub struct PlainStruct {
    pub x: i64,
    y: i64,
}

pub mod my_module {
    // Has a private field inside a module, gains a new pub field.
    pub struct NestedStruct {
        pub x: i64,
        y: i64,
    }
}

// Has a private field, gains two new pub fields so both should be reported.
pub struct StructGainsTwoFields {
    pub x: i64,
    y: i64,
}

// All fields are private, gains a new pub field.
pub struct AllPrivateFields {
    x: i64,
}

// No private fields, so constructible_struct_adds_field covers this as a major change instead.
pub struct ExhaustiveAllPublicStruct {
    pub x: i64,
}

// Has #[non_exhaustive], so non_exhaustive_struct_pub_field_added handles this instead.
#[non_exhaustive]
pub struct NonExhaustiveStruct {
    pub x: i64,
    y: i64,
}

// Not public, so not part of the public API.
struct PrivateStruct {
    pub x: i64,
    y: i64,
}

// pub(crate) is not part of the public API.
pub(crate) struct CrateStruct {
    pub x: i64,
    y: i64,
}

// Already has all fields, so nothing changes.
pub struct UnchangedStruct {
    pub x: i64,
    pub y: i64,
    z: i64,
}

// New field will be private, not pub.
pub struct StructGainsPrivateField {
    pub x: i64,
    y: i64,
}

// A previously private field becomes public while another private field remains.
// This is a different kind of change so this lint should not fire for it.
pub struct PrivateFieldBecomesPublic {
    pub x: i64,
    y: i64,
    z: i64,
}

// The only private field becomes public in the new version and a new pub field is also added.
pub struct PrivateBecomesPubGainsField {
    pub x: i64,
    y: i64,
}

// The only private field is removed in the new version and a new pub field is also added.
pub struct PrivateRemovedGainsField {
    pub x: i64,
    y: i64,
}
