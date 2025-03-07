#![no_std]

pub trait Example<const N: usize> {}

pub trait NotGenericAnymore {}

pub trait NotGenericEither<T> {}
