#![no_std]

pub fn public_fn() {}

fn private_fn() {}

#[doc(hidden)]
pub fn doc_hidden_fn() {}
