#![no_std]

// This should trigger the lint using witnesses
pub fn function_with_parameter_changed(_: i32) {}

// This should be detected by the lint, but should be ignored since it won't trigger the witness
pub fn function_with_signature_changed(_param: ()) {}
