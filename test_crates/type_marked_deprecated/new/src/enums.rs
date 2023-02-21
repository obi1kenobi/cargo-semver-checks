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

// This enum was added in the new version of the crate with its attribute.
// It should NOT be reported by this rule to avoid duplicate lints.
// It should be reported as a new pub type that is part of the crate's API.
// This might seem like a nonsensical test but is a valid edge case.

#[deprecated]
pub enum DeprecatedNewEnum {
    First,
}

