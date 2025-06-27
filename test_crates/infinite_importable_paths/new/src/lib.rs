#![no_std]

//! This package exports the functions:
//! - `foo`
//! - `nested::foo`
//! - `nested::nested::foo`
//! etc. with arbitrarily many repetitions of `nested` allowed.
//!
//! This package uses a different way of achieving the same thing
//! compared to its variant in the `old` directory.
//! No semver issues should be reported when comparing the two.

mod nested_other {
    pub use super::nested;

    pub fn foo() {}
}

pub mod nested {
    pub use crate::nested_other::foo;

    pub use crate::nested_other::nested;
}

pub use nested::foo;
