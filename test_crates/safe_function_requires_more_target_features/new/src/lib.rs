#![no_std]

#[target_feature(enable = "avx")]
#[target_feature(enable = "avx2")]
pub fn safe_function() {}

#[target_feature(enable = "avx")]
#[target_feature(enable = "avx2")]
pub fn second_function() {}

#[target_feature(enable = "avx")]
#[target_feature(enable = "avx2")]
fn private_function() {}

#[target_feature(enable = "avx2")]
#[target_feature(enable = "avx")]
pub fn implied_feature_function() {}

#[target_feature(enable = "bmi1")]
#[target_feature(enable = "sse2")]
pub fn globally_enabled_function() {}

#[target_feature(enable = "avx")]
pub fn replaced_with_narrower_feature() {}

#[target_feature(enable = "avx2")]
pub fn replaced_with_broader_feature() {}

#[target_feature(enable = "bmi1")]
#[target_feature(enable = "bmi2")]
pub unsafe fn becomes_unsafe() {}
