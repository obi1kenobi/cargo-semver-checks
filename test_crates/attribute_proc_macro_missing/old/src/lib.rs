#![no_std]

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn logging(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

// Keep this to verify we don't trigger on function-like macros
#[proc_macro]
pub fn sql(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

// Keep this to verify we don't trigger on derive macros
#[proc_macro_derive(MyDerive)]
pub fn my_derive(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}
