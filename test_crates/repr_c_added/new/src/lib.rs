#![no_std]

#[repr(C)]
pub struct Example {
    pub f1: i32,
    pub f2: i16,
}

#[repr(C)]
pub enum Bar {
    A,
    B,
}

#[repr(C)]
pub union Baz {
    pub x: u32,
    pub y: i32,
}