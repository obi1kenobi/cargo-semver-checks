// STRUCTS

// Adding #[deprecated] to these structs should be reported.

pub struct StructToDeprecatedStruct {
    bar: u64,
}

pub struct StructToDeprecatedMessageStruct {
    bar: u64,
}

// These structs already have the attribute and changes to the attribute will not be reported.

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
