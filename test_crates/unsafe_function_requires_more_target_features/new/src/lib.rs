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

// This function is safe. A different lint should report its extra feature.
#[target_feature(enable = "avx")]
#[target_feature(enable = "avx2")]
pub fn safe_function() {}

// Gained a feature, this should lint.
#[target_feature(enable = "avx")]
#[target_feature(enable = "avx2")]
pub unsafe fn unsafe_function() {}

// Not a public function, shouldn't lint.
#[target_feature(enable = "avx")]
#[target_feature(enable = "avx2")]
unsafe fn private_function() {}

// The new feature is already implied by the existing one, shoudln't lint.
#[target_feature(enable = "avx2")]
#[target_feature(enable = "avx")]
pub unsafe fn implied_feature_function() {}

// The new feature is already globally enabled on the target, shouldn't lint.
#[target_feature(enable = "bmi1")]
#[target_feature(enable = "sse2")]
pub unsafe fn globally_enabled_function() {}

// There was previously a feature, but it was replaced by a narrower feature
// that was already implied by the previous one. This shouldn't lint.
#[target_feature(enable = "avx")]
pub unsafe fn replaced_with_narrower_feature() {}

// There was previously a feature, but it was replaced by a *broader* feature.
// The new feature implies the old, but not vice versa. This should lint.
#[target_feature(enable = "avx2")]
pub unsafe fn replaced_with_broader_feature() {}

// This function became safe while also adding a new feature. It should lint anyway.
#[target_feature(enable = "bmi1")]
#[target_feature(enable = "bmi2")]
pub fn becomes_safe() {}
