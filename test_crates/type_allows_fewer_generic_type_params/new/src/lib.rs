#![no_std]

pub struct Example<T> {
    data: T,
}

pub enum NotGenericAnymore {
    First(i64),
}

pub union NotGenericEither<const N: usize> {
    left: core::mem::ManuallyDrop<[i64; N]>,
}
