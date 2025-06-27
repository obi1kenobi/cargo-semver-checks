#![no_std]

#[non_exhaustive]
pub struct NotGeneric<const COUNT: usize> {
    data: [i64; COUNT],
}

#[non_exhaustive]
pub enum DefaultBecomesRequired<const N: usize, const M: usize = 1> {
    First([i64; N]),
    Second([i64; M]),
}

pub union ConstGenericAdded<T, const N: usize> {
    left: core::mem::ManuallyDrop<[T; N]>,
}

// This one isn't breaking, so it shouldn't be flagged!
pub union DefaultedConstGenericAdded<T, const N: usize = 16> {
    left: core::mem::ManuallyDrop<[T; N]>,
}
