#![no_std]

pub union UnionToStructSingleFieldMismatch {
    pub value: u8,
}

pub union UnionToStructSingleFieldCompatible {
    pub value: u8,
}

pub union UnionToStructPrivateField {
    pub value: u8,
}

pub union UnionToStructNoFields {
    pub value: u8,
}

pub union UnionToStructMultipleFields {
    pub value: u8,
}

pub union UnionToStructDocHiddenField {
    pub value: u8,
}

pub union UnionToStructPrivateExtraField {
    pub value: u8,
}

pub union UnionToNonExhaustiveStruct {
    pub value: u8,
}

pub union UnionWithMultiplePubFields {
    pub first: u8,
    pub second: u16,
}

pub union UnionToHiddenStruct {
    pub value: u8,
}
