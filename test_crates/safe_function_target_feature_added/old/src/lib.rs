#![no_std]

/// Proof that our intended witness is valid on the original version:
/// ```rust
/// fn witness() {
///     let _: Box<dyn Fn(_) -> _> = Box::new(safe_function_target_feature_added::compute);
/// }
/// ```
pub fn compute(x: u32) -> u32 {
    x + 1
}

pub fn will_become_unsafe(x: u32) -> u32 {
    x / 2
}

fn private_fn(x: u32) -> u32 {
    x * 2
}

mod private_module {
    pub fn hidden_fn() {}
}
