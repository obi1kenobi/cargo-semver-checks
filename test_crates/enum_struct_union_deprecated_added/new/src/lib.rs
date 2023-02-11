// STRUCTS

// These structs did not have the #[deprecated] attribute in the old version.
// Addition of the attribute should be reported by this rule.

#[deprecated]
pub struct StructToDeprecatedStruct {
    bar: u64,
}

#[deprecated = "This attribute was added"]
pub struct StructToDeprecatedMessageStruct {
    bar: u64,
}

// These structs had the #[deprecated] attribute in the old version. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

pub struct DeprecatedStructToStruct {
    bar: u64,
}

#[deprecated = "This message was added"]
pub struct DeprecatedStructToDeprecatedMessageStruct {
    bar: u64,
}

// These structs had the #[deprecated] attribute in the old version.
// They also included the user-defined warning message. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

pub struct DeprecatedMessageStructToStruct {
    bar: u64,
}

#[deprecated]
pub struct DeprecatedMessageStructToDeprecatedStruct {
    bar: u64,
}

#[deprecated = "This message was changed"]
pub struct DeprecatedMessageStructToDeprecatedMessageStruct {
    bar: u64,
}

// This struct is private and should NOT be reported by this rule.

#[deprecated]
struct DeprecatedPrivateStruct {
    bar: u64,
}

// This struct was added in the new version of the crate with its attribute.
// It should NOT be reported by this rule to avoid duplicate lints.
// It should be reported as a new pub type that is part of the crate's API.
// This might seem like a nonsensical test but is a valid edge case.

#[deprecated]
pub struct DeprecatedNewStruct {
    bar: u64,
}

// ENUMS

// These enums did not have the #[deprecated] attribute in the old version.
// Addition of the attribute should be reported.

#[deprecated]
pub enum EnumToDeprecatedEnum {
    First,
}

#[deprecated = "This attribute was added"]
pub enum EnumToDeprecatedMessageEnum {
    First,
}

// These structs had the #[deprecated] attribute in the old version.
// They also included the user-defined warning message. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

pub enum DeprecatedEnumToEnum {
    First,
}

#[deprecated = "This message was added"]
pub enum DeprecatedEnumToDeprecatedMessageEnum {
    First,
}

pub enum DeprecatedMessageEnumToEnum {
    First,
}

#[deprecated]
pub enum DeprecatedMessageEnumToDeprecatedEnum {
    First,
}

#[deprecated = "This message was changed"]
pub enum DeprecatedMessageEnumToDeprecatedMessageEnum {
    First,
}

// This enum is private and should NOT be reported.

#[deprecated]
enum DeprecatedPrivateEnum {
    First,
}

// This struct was added in the new version of the crate with its attribute.
// It should NOT be reported by this rule to avoid duplicate lints.
// It should be reported as a new pub type that is part of the crate's API.
// This might seem like a nonsensical test but is a valid edge case.

#[deprecated]
pub struct DeprecatedNewEnum {
    bar: u64,
}
