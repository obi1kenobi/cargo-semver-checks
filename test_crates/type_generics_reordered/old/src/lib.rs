#![no_std]

pub struct OneEachTypeFirst<T, const N: usize>([T; N]);

pub struct OneEachConstFirst<const N: usize, T>([T; N]);

pub struct LifetimeIgnored<'a, const N: usize, T>(&'a [T; N]);

pub struct TwoEachButOneChanged<T, const N: usize, const M: usize, U>([T; N], [U; M]);

pub struct TwoEachTwoChanged<T, const N: usize, const M: usize, U>([T; N], [U; M]);

pub struct LikeForLikeChangedNotBreaking<T, const N: usize, const M: usize, U>([T; N], [U; M]);

struct BreakingButNotPub<T, const N: usize>([T; N]);
