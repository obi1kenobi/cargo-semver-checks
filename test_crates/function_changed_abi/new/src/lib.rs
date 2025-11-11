#![no_std]

pub extern "Rust" fn c_function_becomes_rust() -> () {}

pub fn c_function_becomes_rust_implicitly() -> () {}

pub extern "C" fn rust_function_becomes_c() -> () {}

pub extern "C" fn implicit_rust_function_becomes_c() -> () {}

pub extern "Rust" fn implicit_c_function_becomes_rust() -> () {}

pub extern "Rust" fn rust_function_remains_rust() -> () {}

pub extern "C" fn c_function_remains_c() -> () {}

pub extern "C" fn system_function_becomes_c() -> () {}

extern "Rust" fn private_c_function_becomes_rust() -> () {}

pub extern "Rust" fn implicit_rust_function_becomes_rust() -> () {}

pub fn rust_function_becomes_implicit_rust() -> () {}

pub struct InherentType;

impl InherentType {
    pub extern "Rust" fn inherent_c_becomes_rust() {}
}

pub trait TraitWithAbi {
    extern "Rust" fn trait_c_becomes_rust();
}

#[doc(hidden)]
pub struct DocHiddenInherentType;

impl DocHiddenInherentType {
    // shouldn't trigger the lint due to #[doc(hidden)] type
    pub extern "Rust" fn doc_hidden_inherent_c_becomes_rust() {}
}

pub struct DocHiddenInherentMethod;

impl DocHiddenInherentMethod {
    // shouldn't trigger the lint due to #[doc(hidden)] method
    #[doc(hidden)]
    pub extern "Rust" fn doc_hidden_inherent_c_becomes_rust() {}
}

#[doc(hidden)]
pub trait DocHiddenTraitWithAbi {
    // shouldn't trigger the lint due to #[doc(hidden)] type
    extern "Rust" fn doc_hidden_trait_c_becomes_rust();
}

pub trait DocHiddenMethodInTraitWithAbi {
    // shouldn't trigger the lint due to #[doc(hidden)] method
    #[doc(hidden)]
    extern "Rust" fn doc_hidden_trait_c_becomes_rust();
}
