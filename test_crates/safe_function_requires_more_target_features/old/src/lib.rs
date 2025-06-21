#![no_std]

#[target_feature(enable = "avx")]
pub fn safe_function() {}


#[target_feature(enable = "avx")]
fn private_function() {}

#[target_feature(enable = "avx2")]
pub fn implied_feature_function() {}

#[target_feature(enable = "bmi1")]
pub fn globally_enabled_function() {}

#[target_feature(enable = "avx2")]
pub fn replaced_with_narrower_feature() {}

#[target_feature(enable = "avx")]
pub fn replaced_with_broader_feature() {}

#[target_feature(enable = "bmi1")]
pub fn becomes_unsafe() {}
