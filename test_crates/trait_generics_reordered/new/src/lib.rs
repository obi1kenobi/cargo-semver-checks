#![no_std]

pub trait OneEachTypeFirst<const N: usize, T> {}

pub trait OneEachConstFirst<T, const N: usize> {}

pub trait LifetimeIgnored<'a, T, const N: usize> {}

pub trait TwoEachButOneChanged<T, U, const N: usize, const M: usize> {}

pub trait TwoEachTwoChanged<const N: usize, T, U, const M: usize> {}

pub trait LikeForLikeChangedNotBreaking<U, const M: usize, const N: usize, T> {}

trait NotPublic<const N: usize, T> {}
