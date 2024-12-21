pub fn previously_not_generic() {}

pub fn add_const_generic<const N: usize>(data: [i64; N]) {}

pub fn remove_const_generic<const M: usize>(data: [i64; M]) {}
