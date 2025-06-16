#![no_std]
// ^^^^^^^ This is used to massively decrease the generated rustdoc JSON size.
// You can remove it if your test specifically requires `std` functionality.
//
// Usually, you can avoid depending on `std` by importing the same types from `core` instead:
// `std::fmt::Debug` is the same as `core::fmt::Debug`,
// `std::marker::PhantomData` is the same as `core::marker::PhantomData` etc.
//
// Similarly, unless you specifically need `String` for a test,
// you can usually avoid it by using `&'static str` instead.

#[target_feature(enable = "avx")]
pub fn safe_function() {}

#[target_feature(enable = "avx")]
pub unsafe fn unsafe_function() {}

#[target_feature(enable = "avx")]
fn private_function() {}

#[target_feature(enable = "avx2")]
pub fn implied_feature_function() {}

#[target_feature(enable = "fma")]
pub fn globally_enabled_function() {}

#[target_feature(enable = "avx")]
pub fn replaced_feature_function() {}
