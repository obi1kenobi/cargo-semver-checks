// Adding #[deprecated] to these enums should be reported.

pub enum EnumToDeprecatedEnum {
    First,
}

pub enum EnumToDeprecatedMessageEnum {
    First,
}

// These enums already have the attribute and changes to the attribute should not be reported.

#[deprecated]
pub enum DeprecatedEnumToEnum {
    First,
}

#[deprecated]
pub enum DeprecatedEnumToDeprecatedMessageEnum {
    First,
}

#[deprecated = "This attribute will be deleted"]
pub enum DeprecatedMessageEnumToEnum {
    First,
}

#[deprecated = "This message will be deleted"]
pub enum DeprecatedMessageEnumToDeprecatedEnum {
    First,
}

#[deprecated = "This message will change"]
pub enum DeprecatedMessageEnumToDeprecatedMessageEnum {
    First,
}

// This enum is private and should NOT be reported.

enum DeprecatedPrivateEnum {
    First,
}

