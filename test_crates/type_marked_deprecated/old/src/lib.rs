// STRUCTS

// Adding #[deprecated] to these structs should be reported.

pub struct StructToDeprecatedStruct {
    bar: u64,
}

pub struct StructToDeprecatedMessageStruct {
    bar: u64,
}

// These structs already have the attribute and changes to the attribute should not be reported.

#[deprecated]
pub struct DeprecatedStructToStruct {
    bar: u64,
}

#[deprecated]
pub struct DeprecatedStructToDeprecatedMessageStruct {
    bar: u64,
}

#[deprecated = "This attribute will be deleted"]
pub struct DeprecatedMessageStructToStruct {
    bar: u64,
}

#[deprecated = "This message will be deleted"]
pub struct DeprecatedMessageStructToDeprecatedStruct {
    bar: u64,
}

#[deprecated = "This message will change"]
pub struct DeprecatedMessageStructToDeprecatedMessageStruct {
    bar: u64,
}

// This struct is private and should NOT be reported.

struct DeprecatedPrivateStruct {
    bar: u64,
}

// ENUMS

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

// UNIONS

// Adding #[deprecated] to these unions should be reported.

pub union UnionToDeprecatedUnion {
    bar: u64,
}

pub union UnionToDeprecatedMessageUnion {
    bar: u64,
}

// These unions already have the attribute and changes to the attribute should not be reported.

#[deprecated]
pub union DeprecatedUnionToUnion {
    bar: u64,
}

#[deprecated]
pub union DeprecatedUnionToDeprecatedMessageUnion {
    bar: u64,
}

#[deprecated = "This attribute will be deleted"]
pub union DeprecatedMessageUnionToUnion {
    bar: u64,
}

#[deprecated = "This message will be deleted"]
pub union DeprecatedMessageUnionToDeprecatedUnion {
    bar: u64,
}

#[deprecated = "This message will change"]
pub union DeprecatedMessageUnionToDeprecatedMessageUnion {
    bar: u64,
}

// This union is private and should NOT be reported.

union DeprecatedPrivateUnion {
    bar: u64,
}
