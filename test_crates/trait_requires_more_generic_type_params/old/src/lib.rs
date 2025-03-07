#![no_std]

pub trait NotGeneric {}

pub trait DefaultBecomesRequired<A, B = i64> {}

pub trait GenericAdded<T> {}

// This one isn't breaking, so it shouldn't be flagged!
pub trait DefaultedGenericAdded<T> {}
