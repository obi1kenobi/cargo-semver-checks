#![no_std]

pub trait Example<A> {}

pub trait NotGenericAnymore {}

pub trait NotGenericEither<const N: usize> {}
