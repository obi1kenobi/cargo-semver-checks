#![no_std]

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn identity(item: TokenStream) -> TokenStream {
    item
}
