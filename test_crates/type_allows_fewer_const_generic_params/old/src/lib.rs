#![no_std]

pub struct Example<const N: usize, const M: usize = 8> {
    data: [i64; N],
    data2: [i64; M],
}

pub enum NotGenericAnymore<const N: usize> {
    First([i64; N]),
}

pub union NotGenericEither<T, const N: usize> {
    left: core::mem::ManuallyDrop<[T; N]>,
}
