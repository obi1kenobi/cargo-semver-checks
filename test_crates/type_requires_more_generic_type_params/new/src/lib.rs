#[non_exhaustive]
pub struct NotGeneric<T> {
    data: T,
}

#[non_exhaustive]
pub enum DefaultBecomesRequired<A, B = String> {
    First(A),
    Second(B),
}

pub union GenericAdded<T, const N: usize = 16> {
    left: std::mem::ManuallyDrop<[T; N]>,
}

// This one isn't breaking, so it shouldn't be flagged!
pub union DefaultedTypeGenericAdded<const N: usize, T = i64> {
    left: std::mem::ManuallyDrop<[T; N]>,
}
