#![no_std]

pub fn previously_not_generic<const N: usize>() -> [i64; N] {
    todo!()
}

pub fn add_const_generic<const N: usize, const M: usize>(data: [i64; N]) -> [i64; M] {
    todo!()
}

pub fn remove_const_generic(data: [i64; 8]) {}
