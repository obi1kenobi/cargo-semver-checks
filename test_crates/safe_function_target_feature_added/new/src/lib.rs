#![no_std]

#[target_feature(enable = "sse2")]
pub fn compute(x: u32) -> u32 {
    x + 1
}

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
