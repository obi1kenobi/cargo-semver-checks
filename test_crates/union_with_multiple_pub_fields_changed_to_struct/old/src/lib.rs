#![no_std]

pub union UnionWithMultiplePubFields {
    pub first: u8,
    pub second: u16,
}

pub union UnionWithOnePubField {
    pub value: u8,
}
