#![no_std]

pub trait NotGeneric<const COUNT: usize> {}

pub trait DefaultBecomesRequired<const N: usize, const M: usize = 1> {}

pub trait ConstGenericAdded<T, const N: usize> {}

// This one isn't breaking, so it shouldn't be flagged!
pub trait DefaultedConstGenericAdded<T, const N: usize = 16> {}
