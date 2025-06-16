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
