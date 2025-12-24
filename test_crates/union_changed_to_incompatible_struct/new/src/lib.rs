#![no_std]

pub struct UnionToStructSingleFieldMismatch {
    pub other: u8,
}

pub struct UnionToStructSingleFieldCompatible {
    pub value: u8,
}

pub struct UnionToStructPrivateField {
    value: u8,
}

pub struct UnionToStructNoFields;

pub struct UnionToStructMultipleFields {
    pub value: u8,
    pub extra: u16,
}

pub struct UnionToStructDocHiddenField {
    #[doc(hidden)]
    pub value: u8,
}

pub struct UnionToStructPrivateExtraField {
    pub value: u8,
    extra: u16,
}

#[non_exhaustive]
pub struct UnionToNonExhaustiveStruct {
    pub value: u8,
}

pub struct UnionWithMultiplePubFields {
    pub first: u8,
}

#[doc(hidden)]
pub struct UnionToHiddenStruct {
    pub value: u8,
}
