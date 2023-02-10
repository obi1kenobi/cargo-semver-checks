// These structs did not have the #[deprecated] attribute in the old version.
// Addition of the attribute should be reported by this rule.

pub struct StructToDeprecatedStruct {
    bar: u64,
}

pub struct StructToDeprecatedMessageStruct {
    bar: u64,
}

// These structs had the #[deprecated] attribute in the old version. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

#[deprecated]
pub struct DeprecatedStructToStruct {
    bar: u64,
}

#[deprecated]
pub struct DeprecatedStructToDeprecatedMessageStruct {
    bar: u64,
}

// These structs had the #[deprecated] attribute in the old version.
// They also included the user-defined warning message. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

#[deprecated = "Foo"]
pub struct DeprecatedMessageStructToStruct {
    bar: u64,
}

#[deprecated = "Foo"]
pub struct DeprecatedMessageStructToDeprecatedStruct {
    bar: u64,
}

#[deprecated = "Foo"]
pub struct DeprecatedMessageStructToDeprecatedMessageStruct {
    bar: u64,
}

// This struct is private and should NOT be reported by this rule.

struct DeprecatedPrivateStruct {
    bar: u64,
}
