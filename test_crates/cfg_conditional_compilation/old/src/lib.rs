#![no_std]

#[cfg(custom)]
pub struct Example;

pub enum Data {
    Int(i64),
    String(&'static str),
}
