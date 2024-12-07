use proc_macro::TokenStream;

// Helper attribute removed - should trigger
#[proc_macro_derive(MyDerive)]
pub fn my_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

// Unchanged - should not trigger
#[proc_macro_derive(UnchangedDerive, attributes(unchanged))]
pub fn unchanged_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

// Different macro type unchanged - should not trigger
#[proc_macro_attribute]
pub fn my_attribute(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    TokenStream::new()
}
