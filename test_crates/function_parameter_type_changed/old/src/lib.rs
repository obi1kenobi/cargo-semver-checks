#![no_std]

// This should trigger the lint using witnesses
pub fn function_with_parameter_changed(_: i32) {}

// This should be detected by the lint, but should be ignored since it won't trigger the witness
pub fn function_with_signature_changed(_param: ()) {}

// This should be detected by the lint, but should be ignored since i64 is compatible with impl Into<i64>
pub fn explicit_arg_to_impl_trait(_: i64) {}

// This should be detected by the lint, but should be ignored since Into<i64> is being extracted to a generic
pub fn impl_trait_to_generic(_: impl Into<i64>) {}

// This should be detected by the lint, but should be ignored since it is a symmetric non-breaking change
pub fn impl_trait_to_generic_where_bound(_: impl Into<i64>) {}
