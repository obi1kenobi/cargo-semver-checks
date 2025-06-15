#![no_std]

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
