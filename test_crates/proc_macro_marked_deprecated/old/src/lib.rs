#![no_std]

use proc_macro::TokenStream;

// Function-like proc macros that will be marked deprecated
#[proc_macro]
pub fn proc_macro_to_deprecated(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro]
pub fn proc_macro_to_deprecated_message(input: TokenStream) -> TokenStream {
    input
}

// Derive macros that will be marked deprecated
#[proc_macro_derive(DeriveToDeprecated)]
pub fn derive_to_deprecated(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_derive(DeriveToDeprecatedMessage)]
pub fn derive_to_deprecated_message(input: TokenStream) -> TokenStream {
    input
}

// Attribute macros that will be marked deprecated
#[proc_macro_attribute]
pub fn attr_to_deprecated(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

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
#[deprecated = "Old message"]
#[proc_macro]
pub fn proc_macro_message_changes(input: TokenStream) -> TokenStream {
    input
}

#[deprecated = "Old message"]
#[proc_macro_derive(DeriveMessageChanges)]
pub fn derive_message_changes(input: TokenStream) -> TokenStream {
    input
}

#[deprecated = "Old message"]
#[proc_macro_attribute]
pub fn attr_message_changes(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

// Hidden proc macros should not be reported
#[doc(hidden)]
#[proc_macro]
pub fn hidden_proc_macro_to_deprecated(input: TokenStream) -> TokenStream {
    input
}

#[doc(hidden)]
#[proc_macro_derive(HiddenDeriveToDeprecated)]
pub fn hidden_derive_to_deprecated(input: TokenStream) -> TokenStream {
    input
}

#[doc(hidden)]
#[proc_macro_attribute]
pub fn hidden_attr_to_deprecated(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
