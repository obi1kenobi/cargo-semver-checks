#![no_std]

// These structs did not have the #[must_use] attribute in the old version.
// Addition of the attribute should be reported by this rule.

#[must_use]
pub struct StructToMustUseStruct {
    bar: u64,
}

#[must_use = "Foo"]
pub struct StructToMustUseMessageStruct {
    bar: u64,
}


// These structs had the #[must_use] attribute in the old version. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

pub struct MustUseStructToStruct {
    bar: u64,
}

#[must_use = "Foo"]
pub struct MustUseStructToMustUseMessageStruct {
    bar: u64,
}


// These structs had the #[must_use] attribute in the old version.
// They also included the user-defined warning message. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

pub struct MustUseMessageStructToStruct {
    bar: u64,
}

#[must_use]
pub struct MustUseMessageStructToMustUseStruct {
    bar: u64,
}

#[must_use = "Baz"]
pub struct MustUseMessageStructToMustUseMessageStruct {
    bar: u64,
}


// This struct is private and should NOT be reported by this rule.

#[must_use]
struct MustUsePrivateStruct {
    bar: u64,
}


// This struct was added in the new version of the crate with its attribute.
// It should NOT be reported by this rule to avoid duplicate lints.
// It should be reported as a new pub type that is part of the crate's API.

#[must_use]
pub struct MustUseNewStruct {
    bar: u64,
}
