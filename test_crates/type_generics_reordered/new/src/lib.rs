#![no_std]

pub struct OneEachTypeFirst<const N: usize, T>([T; N]);

pub struct OneEachConstFirst<T, const N: usize>([T; N]);

pub struct LifetimeIgnored<'a, T, const N: usize>(&'a [T; N]);

pub struct TwoEachButOneChanged<T, U, const N: usize, const M: usize>([T; N], [U; M]);

pub struct TwoEachTwoChanged<const N: usize, T, U, const M: usize>([T; N], [U; M]);

pub struct LikeForLikeChangedNotBreaking<U, const M: usize, const N: usize, T>([U; M], [T; N]);

struct BreakingButNotPub<const N: usize, T>([T; N]);
