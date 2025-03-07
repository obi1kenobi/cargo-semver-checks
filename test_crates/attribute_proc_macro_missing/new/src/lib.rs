#![no_std]

use proc_macro::TokenStream;

// Keep other macro types to verify specific detection
#[proc_macro]
pub fn sql(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_derive(MyDerive)]
pub fn my_derive(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}
