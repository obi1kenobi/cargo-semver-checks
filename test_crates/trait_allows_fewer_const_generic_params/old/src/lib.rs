pub trait Example<const N: usize, const M: usize = 8> {}

pub trait NotGenericAnymore<const N: usize> {}

pub trait NotGenericEither<T, const N: usize> {}
