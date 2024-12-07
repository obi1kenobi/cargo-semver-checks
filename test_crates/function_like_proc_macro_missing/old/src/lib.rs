use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

// Will keep this one to ensure we don't have false positives
#[proc_macro]
pub fn another_macro(_item: TokenStream) -> TokenStream {
    "fn other() -> u32 { 0 }".parse().unwrap()
}

// Other kinds of proc macros that should not trigger this lint
#[proc_macro_derive(MyDerive)]
pub fn my_derive(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn my_attribute(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    TokenStream::new()
}