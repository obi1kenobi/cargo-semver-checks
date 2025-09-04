#![no_std]

#[repr(align(16))]
pub struct StructAlignChanged;

#[repr(align(32))]
pub enum EnumAlignChanged {
    A,
}

#[repr(align(64))]
pub union UnionAlignChanged {
    a: u8,
}

#[repr(align(16))]
pub struct StructAlignUnchanged;

#[repr(align(16))]
struct WasPub;

mod reexported_mod {
    #[repr(align(16))]
    pub struct Reexported;
}
// `Reexported` is no longer re-exported
