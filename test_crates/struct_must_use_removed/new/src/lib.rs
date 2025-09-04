#![no_std]

// These structs had the #[must_use] attribute in the old version.
// Removal of the attribute should be reported by this rule.

pub struct MustUseStructToStruct {
    bar: u64,
}

pub struct MustUseMessageStructToStruct {
    bar: u64,
}

// These structs did not have the #[must_use] attribute in the old version.
// Addition of the attribute should NOT be reported by this rule.

#[must_use]
pub struct StructToMustUseStruct {
    bar: u64,
}

#[must_use = "Foo"]
pub struct StructToMustUseMessageStruct {
    bar: u64,
}

// These structs change from one form of #[must_use] to another.
// They should NOT be reported by this rule.

#[must_use = "Foo"]
pub struct MustUseStructToMustUseMessageStruct {
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

struct MustUsePrivateStruct {
    bar: u64,
}
