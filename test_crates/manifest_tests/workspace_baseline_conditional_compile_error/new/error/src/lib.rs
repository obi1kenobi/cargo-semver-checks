#![no_std]

//! Has a compile error in the baseline version with default features disabled,
//! so we can't generate the rustdoc JSON to
//! run semver-checks on

// in baseline:
// #[cfg(feature = "some_feature")]
// compile_error!("This crate has a compiler error.");
