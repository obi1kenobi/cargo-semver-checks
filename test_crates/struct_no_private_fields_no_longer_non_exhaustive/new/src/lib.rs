#![no_std]

pub struct NoFields;
pub struct NoFieldsTuple();
pub struct NoFieldsStruct {}

pub struct Tuple(pub u8);
pub struct Struct {
    pub inner: u8,
    pub another: Tuple,
}

pub struct WithPrivate {
    pub(self) inner: u8,
}
