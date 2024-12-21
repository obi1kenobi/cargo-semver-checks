pub struct Owner;

impl Owner {
    pub fn previously_not_generic() {}

    pub fn added_const_generic<const N: usize>(data: [i64; N]) {}

    pub fn removed_const_generic<const N: usize, const M: usize>(data: [i64; N]) -> [i64; M] {
        todo!()
    }
}
