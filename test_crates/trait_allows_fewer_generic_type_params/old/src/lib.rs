#![no_std]

pub trait Example<A, B = i64> {}

pub trait NotGenericAnymore<T> {}

pub trait NotGenericEither<const N: usize, T> {}
