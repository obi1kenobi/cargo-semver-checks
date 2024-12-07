use proc_macro::TokenStream;

// Test case 1: Will have helper attribute removed
#[proc_macro_derive(MyDerive, attributes(helper))]
pub fn my_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

// Test case 2: Will remain unchanged (prevent false positives)
#[proc_macro_derive(UnchangedDerive, attributes(unchanged))]
pub fn unchanged_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

// Test case 3: Different macro type (prevent false positives)
#[proc_macro_attribute]
pub fn my_attribute(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    TokenStream::new()
}
