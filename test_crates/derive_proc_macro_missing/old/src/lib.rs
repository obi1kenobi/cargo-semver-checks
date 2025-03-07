#![no_std]

use proc_macro::TokenStream;

#[proc_macro_derive(MyDerive)]
pub fn my_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_derive(StaysPresent)]
pub fn stays_present(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn my_attribute(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro]
pub fn my_macro(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
