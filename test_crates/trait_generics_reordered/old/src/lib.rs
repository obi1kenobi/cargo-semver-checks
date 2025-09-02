#![no_std]

pub trait OneEachTypeFirst<T, const N: usize> {}

pub trait OneEachConstFirst<const N: usize, T> {}

pub trait LifetimeIgnored<'a, const N: usize, T> {}

pub trait TwoEachButOneChanged<T, const N: usize, const M: usize, U> {}

pub trait TwoEachTwoChanged<T, const N: usize, const M: usize, U> {}

pub trait LikeForLikeChangedNotBreaking<T, const N: usize, const M: usize, U> {}

trait NotPublic<T, const N: usize> {}
