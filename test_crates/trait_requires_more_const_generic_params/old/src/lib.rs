#![no_std]

pub trait NotGeneric {}

pub trait DefaultBecomesRequired<const N: usize = 1, const M: usize = 1> {}

pub trait ConstGenericAdded<T> {}

// This one isn't breaking, so it shouldn't be flagged!
pub trait DefaultedConstGenericAdded<T> {}
