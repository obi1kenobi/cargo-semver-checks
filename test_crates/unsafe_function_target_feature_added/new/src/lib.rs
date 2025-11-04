#![no_std]

// Unsafe function now explicitly requires avx2.
#[target_feature(enable = "avx2")]
pub unsafe fn feature_added() {}

// Function already had avx2 requirement. Unchanged.
#[target_feature(enable = "avx2")]
pub unsafe fn already_featured() {}

// Adds a feature that is globally enabled (sse2). Should not trigger.
#[target_feature(enable = "sse2")]
pub unsafe fn add_globally_enabled_feature() {}

// Private unsafe function adds feature.
#[target_feature(enable = "avx2")]
unsafe fn private_feature_added() {}

// The function became safe *and* added a non-implied, non-globally-enabled feature.
// It should trigger the lint.
#[target_feature(enable = "avx")]
pub fn becomes_safe() {}
