#![no_std]

// Unsafe function without target features. Adding a feature should trigger.
pub unsafe fn feature_added() {}

// Unsafe function already has a target feature. Updating features should not trigger.
#[target_feature(enable = "avx2")]
pub unsafe fn already_featured() {}

// Unsafe function that will add a globally enabled feature in new version.
pub unsafe fn add_globally_enabled_feature() {}

// Private unsafe function adding a feature should not trigger.
unsafe fn private_feature_added() {}

// This function becomes safe and adds target features. It should trigger the lint.
pub unsafe fn becomes_safe() {}
