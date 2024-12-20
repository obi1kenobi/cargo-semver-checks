pub struct Example<const N: usize> {
    data: [i64; N],
}

pub enum NotGenericAnymore {
    First([i64; 16]),
}

pub union NotGenericEither<T> {
    left: std::mem::ManuallyDrop<[T; 4]>,
}
