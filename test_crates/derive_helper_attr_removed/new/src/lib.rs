#![no_std]

use proc_macro::TokenStream;

// Helper attribute removed - should trigger
#[proc_macro_derive(MyDerive)]
pub fn my_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
