pub trait NotGeneric<T> {}

pub trait DefaultBecomesRequired<A, B> {}

pub trait GenericAdded<T, U> {}

// This one isn't breaking, so it shouldn't be flagged!
pub trait DefaultedGenericAdded<T, U = String> {}
