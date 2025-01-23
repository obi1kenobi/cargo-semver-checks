#[non_exhaustive]
pub struct NotGeneric {}

#[non_exhaustive]
pub enum DefaultBecomesRequired<A = i64, B = String> {
    First(A),
    Second(B),
}

pub union GenericAdded<const N: usize = 16> {
    left: std::mem::ManuallyDrop<[i64; N]>,
}

// This one isn't breaking, so it shouldn't be flagged!
pub union DefaultedTypeGenericAdded<const N: usize> {
    left: std::mem::ManuallyDrop<[i64; N]>,
}
