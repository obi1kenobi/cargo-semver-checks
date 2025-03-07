#![no_std]

pub enum Data {
    Int(i64),
    String(&'static str),
    #[cfg(custom)]
    Bool(bool),
}
