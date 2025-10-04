#![no_std]

use proc_macro::TokenStream;

// Test case 1: Will have helper attribute removed
#[proc_macro_derive(MyDerive, attributes(helper))]
pub fn my_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
