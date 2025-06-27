#![no_std]

pub struct Owner;

impl Owner {
    pub fn previously_not_generic<const N: usize>() -> [i64; N] {
        todo!()
    }

    pub fn added_const_generic<const N: usize, const M: usize>(data: [i64; N]) -> [i64; M] {
        todo!()
    }

    pub fn removed_const_generic<const N: usize>(data: [i64; N]) -> [i64; 4] {
        todo!()
    }
}
