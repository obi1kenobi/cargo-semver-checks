#![no_std]

// These structs did not have the #[must_use] attribute in the old version.
// Addition of the attribute should be reported by this rule.

pub struct StructToMustUseStruct {
    bar: u64,
}

pub struct StructToMustUseMessageStruct {
    bar: u64,
}


// These structs had the #[must_use] attribute in the old version. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

#[must_use]
pub struct MustUseStructToStruct {
    bar: u64,
}

#[must_use]
pub struct MustUseStructToMustUseMessageStruct {
    bar: u64,
}


// These structs had the #[must_use] attribute in the old version.
// They also included the user-defined warning message. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

#[must_use = "Foo"]
pub struct MustUseMessageStructToStruct {
    bar: u64,
}

#[must_use = "Foo"]
pub struct MustUseMessageStructToMustUseStruct {
    bar: u64,
}

#[must_use = "Foo"]
pub struct MustUseMessageStructToMustUseMessageStruct {
    bar: u64,
}


// This struct is private and should NOT be reported by this rule.

struct MustUsePrivateStruct {
    bar: u64,
}
