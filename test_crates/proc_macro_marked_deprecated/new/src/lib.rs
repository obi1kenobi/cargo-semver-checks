#![no_std]

use proc_macro::TokenStream;

// These proc macros are now deprecated and should be reported
#[deprecated]
#[proc_macro]
pub fn proc_macro_to_deprecated(input: TokenStream) -> TokenStream {
    input
}

#[deprecated = "This proc macro is deprecated"]
#[proc_macro]
pub fn proc_macro_to_deprecated_message(input: TokenStream) -> TokenStream {
    input
}

#[deprecated]
#[proc_macro_derive(DeriveToDeprecated)]
pub fn derive_to_deprecated(input: TokenStream) -> TokenStream {
    input
}

#[deprecated = "This derive macro is deprecated"]
#[proc_macro_derive(DeriveToDeprecatedMessage)]
pub fn derive_to_deprecated_message(input: TokenStream) -> TokenStream {
    input
}

#[deprecated]
#[proc_macro_attribute]
pub fn attr_to_deprecated(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[deprecated = "This attribute macro is deprecated"]
#[proc_macro_attribute]
pub fn attr_to_deprecated_message(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

// These proc macros should not trigger the lint as they're already deprecated
#[deprecated]
#[proc_macro]
pub fn proc_macro_already_deprecated(input: TokenStream) -> TokenStream {
    input
}

#[deprecated]
#[proc_macro_derive(DeriveAlreadyDeprecated)]
pub fn derive_already_deprecated(input: TokenStream) -> TokenStream {
    input
}

#[deprecated]
#[proc_macro_attribute]
pub fn attr_already_deprecated(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

// These proc macros should not trigger the lint as they're changing message only
#[deprecated = "New message"]
#[proc_macro]
pub fn proc_macro_message_changes(input: TokenStream) -> TokenStream {
    input
}

#[deprecated = "New message"]
#[proc_macro_derive(DeriveMessageChanges)]
pub fn derive_message_changes(input: TokenStream) -> TokenStream {
    input
}

#[deprecated = "New message"]
#[proc_macro_attribute]
pub fn attr_message_changes(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

// Hidden proc macros should not be reported
#[doc(hidden)]
#[deprecated]
#[proc_macro]
pub fn hidden_proc_macro_to_deprecated(input: TokenStream) -> TokenStream {
    input
}

#[doc(hidden)]
#[deprecated]
#[proc_macro_derive(HiddenDeriveToDeprecated)]
pub fn hidden_derive_to_deprecated(input: TokenStream) -> TokenStream {
    input
}

#[doc(hidden)]
#[deprecated]
#[proc_macro_attribute]
pub fn hidden_attr_to_deprecated(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
