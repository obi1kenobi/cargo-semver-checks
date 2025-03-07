#![no_std]

#[cfg(feature = "A")]
pub fn moving_from_feature_A_to_feature_B() {}

#[cfg(feature = "B")]
pub fn moving_from_feature_B_to_feature_C() {}
