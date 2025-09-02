#![no_std]

pub struct Foo;

impl Foo {
    pub fn one_each_type_first<const N: usize, T>(_value: [T; N]) {}

    pub fn one_each_const_first<T, const N: usize>(_value: [T; N]) {}

    pub fn lifetime_ignored<'a, T, const N: usize>(_value: &'a [T; N]) {}

    pub fn two_each_but_one_changed<T, U, const N: usize, const M: usize>(_x: [T; N], _y: [U; M]) {}

    pub fn two_each_two_changed<const N: usize, T, U, const M: usize>(_x: [T; N], _y: [U; M]) {}

    pub fn like_for_like_changed_not_breaking<U, const M: usize, const N: usize, T>(
        _x: [U; M],
        _y: [T; N],
    ) {
    }

    fn breaking_but_not_pub<const N: usize, T>(_value: [T; N]) {}
}
