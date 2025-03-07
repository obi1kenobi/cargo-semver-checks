#![no_std]

#[non_exhaustive]
pub struct NotGeneric {}

#[non_exhaustive]
pub enum DefaultBecomesRequired<const N: usize = 1, const M: usize = 1> {
    First([i64; N]),
    Second([i64; M]),
}

pub union ConstGenericAdded<T> {
    left: core::mem::ManuallyDrop<[T; 16]>,
}

// This one isn't breaking, so it shouldn't be flagged!
pub union DefaultedConstGenericAdded<T> {
    left: core::mem::ManuallyDrop<[T; 16]>,
}
