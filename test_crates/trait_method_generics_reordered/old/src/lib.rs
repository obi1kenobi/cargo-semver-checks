#![no_std]

pub trait Example {
    fn one_each_type_first<T, const N: usize>(_value: [T; N]);
    fn one_each_const_first<const N: usize, T>(_value: [T; N]);
    fn lifetime_ignored<'a, const N: usize, T>(_value: &'a [T; N]);
    fn two_each_but_one_changed<T, const N: usize, const M: usize, U>(_x: [T; N], _y: [U; M]);
    fn two_each_two_changed<T, const N: usize, const M: usize, U>(_x: [T; N], _y: [U; M]);
    fn like_for_like_changed_not_breaking<T, const N: usize, const M: usize, U>(
        _x: [T; N],
        _y: [U; M],
    );
}

trait NotPublic {
    fn breaking_but_not_pub<T, const N: usize>(_value: [T; N]);
}
