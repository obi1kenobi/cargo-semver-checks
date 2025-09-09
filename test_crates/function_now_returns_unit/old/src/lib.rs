#![no_std]

pub fn public_fn() -> i32 {
    42
}

fn private_fn() -> i32 {
    42
}

#[doc(hidden)]
pub fn doc_hidden_fn() -> i32 {
    42
}
