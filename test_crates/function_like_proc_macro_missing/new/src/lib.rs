use proc_macro::TokenStream;

// make_answer is removed

// Keep this to verify we don't have false positives
#[proc_macro]
pub fn another_macro(_item: TokenStream) -> TokenStream {
    "fn other() -> u32 { 0 }".parse().unwrap()
}

// Keep other macro types to verify we don't detect their changes
#[proc_macro_derive(MyDerive)]
pub fn my_derive(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn my_attribute(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    TokenStream::new()
}