#![no_std]

#[non_exhaustive]
pub struct NoFields;
#[non_exhaustive]
pub struct NoFieldsTuple();
#[non_exhaustive]
pub struct NoFieldsStruct {}

#[non_exhaustive]
pub struct Tuple(pub u8);
#[non_exhaustive]
pub struct Struct {
    pub inner: u8,
    pub another: Tuple,
}

#[non_exhaustive]
pub struct WithPrivate {
    pub(self) inner: u8,
}
