use proc_macro::TokenStream;

// An attribute macro we'll hide. Should trigger the lint.
#[doc(hidden)]
#[proc_macro_attribute]
pub fn logging(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

// A function-like macro we'll hide. Should trigger the lint.
#[doc(hidden)]
#[proc_macro]
pub fn sql(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

// A derive macro we'll hide. Should trigger the lint.
#[doc(hidden)]
#[proc_macro_derive(MyDerive)]
pub fn my_derive(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}
