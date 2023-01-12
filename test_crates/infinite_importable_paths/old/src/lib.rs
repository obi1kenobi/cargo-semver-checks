//! This package exports the functions:
//! - `foo`
//! - `nested::foo`
//! - `nested::nested::foo`
//! etc. with arbitrarily many repetitions of `nested` allowed.

pub mod nested {
    pub use super::nested;

    pub fn foo() {}
}

pub use nested::foo;
