#![no_std]

// should trigger lint as it gains #[repr(transparent)]
pub struct StructGainsTransparent(pub i32);

// should trigger lint as it gains repr(transparent)
pub enum EnumGainsTransparent {
    V(i32),
}

// no lints expected as it does not have repr(transparent)
pub struct StructStaysOpaque(pub i32);

// no lints expected as it already has repr(transparent)
#[repr(transparent)]
pub struct StructStaysTransparent(pub i32);

// no lints expected as it is private
struct PrivateStructGainsTransparent(i32);
