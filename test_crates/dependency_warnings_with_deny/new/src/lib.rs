//! Regression fixture for checking semver under `RUSTFLAGS=-Dwarnings`.
//!
//! The public API is intentionally identical between `old` and `new`.
//! The important part of this fixture is its local path dependency, which
//! emits a warning unless cargo-semver-checks caps dependency lints while
//! generating rustdoc JSON.

#![no_std]

pub fn value_from_dependency() -> usize {
    warning_dependency::warning_value()
}
