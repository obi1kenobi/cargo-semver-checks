#![no_std]

pub trait Example {
    fn one_each_type_first<const N: usize, T>(_value: [T; N]);
    fn one_each_const_first<T, const N: usize>(_value: [T; N]);
    fn lifetime_ignored<'a, T, const N: usize>(_value: &'a [T; N]);
    fn two_each_but_one_changed<T, U, const N: usize, const M: usize>(_x: [T; N], _y: [U; M]);
    fn two_each_two_changed<const N: usize, T, U, const M: usize>(_x: [T; N], _y: [U; M]);
    fn like_for_like_changed_not_breaking<U, const M: usize, const N: usize, T>(
        _x: [U; M],
        _y: [T; N],
    );
}

trait NotPublic {
    fn breaking_but_not_pub<const N: usize, T>(_value: [T; N]);
}
