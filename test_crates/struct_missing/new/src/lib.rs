#![no_std]

pub mod my_pub_mod {
    pub struct PubUseRemovedStruct;
}

// This struct is not removed, it only changes kind from tuple to plain.
// It should not be reported as missing.
pub struct ChangeStructKind {
    foo: u64
}
