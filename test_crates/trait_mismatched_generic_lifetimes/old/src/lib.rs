#![no_std]

// All these changes are breaking.
//
// The best witness is using the trait in a bound:
// ```
// fn witness<T: Example>() {}
// ```
// where supplying any number other than the exact number of lifetime parameters is an error:
// ```
//   |
// 6 | fn witness<T: Example>() {}
//   |               ^^^^^^^ expected 2 lifetime parameters
//   |
//   = note: for more information on higher-ranked polymorphism, visit https://doc.rust-lang.org/nomicon/hrtb.html
// help: consider making the bound lifetime-generic with a new `'a` lifetime
// ```

pub trait NotGeneric {}

pub trait GainsLifetimes<'a> {}

pub trait LosesLifetimes<'a, 'b> {}

pub trait StopsBeingGeneric<'a, 'b> {}
