#![no_std]

pub struct Example {
    pub f1: i32,
    pub f2: i16,
}

pub enum Bar {
    A,
    B,
}

pub union Baz {
    pub x: u32,
    pub y: i32,
}