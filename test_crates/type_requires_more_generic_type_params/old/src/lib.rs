#![no_std]

#[non_exhaustive]
pub struct NotGeneric {}

#[non_exhaustive]
pub enum DefaultBecomesRequired<A = i64, B = &'static str> {
    First(A),
    Second(B),
}

pub union GenericAdded<const N: usize = 16> {
    left: core::mem::ManuallyDrop<[i64; N]>,
}

// This one isn't breaking, so it shouldn't be flagged!
pub union DefaultedTypeGenericAdded<const N: usize> {
    left: core::mem::ManuallyDrop<[i64; N]>,
}
