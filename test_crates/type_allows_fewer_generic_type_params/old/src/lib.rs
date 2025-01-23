pub struct Example<A, B> {
    data: A,
    data2: B,
}

pub enum NotGenericAnymore<T> {
    First(T),
}

pub union NotGenericEither<const N: usize, T> {
    left: std::mem::ManuallyDrop<[T; N]>,
}
