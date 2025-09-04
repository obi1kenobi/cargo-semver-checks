#![no_std]

#[repr(align(8))]
pub struct StructAlignChanged;

#[repr(align(8))]
pub enum EnumAlignChanged {
    A,
}

#[repr(align(8))]
pub union UnionAlignChanged {
    a: u8,
}

#[repr(align(16))]
pub struct StructAlignUnchanged;

#[repr(align(8))]
pub struct WasPub;

mod reexported_mod {
    #[repr(align(8))]
    pub struct Reexported;
}

pub use reexported_mod::Reexported;
