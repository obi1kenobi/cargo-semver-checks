#![no_std]

/// Proof that our witness demonstrates breakage:
/// ```compile_fail
/// fn witness() {
///     let _: Box<dyn Fn(_) -> _> = Box::new(safe_function_target_feature_added::compute);
/// }
/// ```
#[target_feature(enable = "sse2")]
pub fn compute(x: u32) -> u32 {
    x + 1
}

// Making a safe function `unsafe` is a breaking change regardless of `#[target_feature]`.
// We have a lint for that, let it take precedence.
#[target_feature(enable = "sse2")]
pub unsafe fn will_become_unsafe(x: u32) -> u32 {
    x / 2
}

#[target_feature(enable = "sse2")]
fn private_fn(x: u32) -> u32 {
    x * 2
}

mod private_module {
    #[target_feature(enable = "sse2")]
    pub fn hidden_fn() {}
}
