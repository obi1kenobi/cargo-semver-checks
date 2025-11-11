#![no_std]

pub extern "C" fn c_function_becomes_rust() -> () {}

pub extern "C" fn c_function_becomes_rust_implicitly() -> () {}

pub extern "Rust" fn rust_function_becomes_c() -> () {}

pub fn implicit_rust_function_becomes_c() -> () {}

#[allow(missing_abi)]
pub extern fn implicit_c_function_becomes_rust() -> () {}

pub extern "Rust" fn rust_function_remains_rust() -> () {}

pub extern "C" fn c_function_remains_c() -> () {}

pub extern "system" fn system_function_becomes_c() -> () {}

extern "C" fn private_c_function_becomes_rust() -> () {}

pub fn implicit_rust_function_becomes_rust() -> () {}

pub extern "Rust" fn rust_function_becomes_implicit_rust() -> () {}

pub struct InherentType;

impl InherentType {
    pub extern "C" fn inherent_c_becomes_rust() {}
}

pub trait TraitWithAbi {
    extern "C" fn trait_c_becomes_rust();
}

#[doc(hidden)]
pub struct DocHiddenInherentType;

impl DocHiddenInherentType {
    // shouldn't trigger the lint due to #[doc(hidden)] type
    pub extern "C" fn doc_hidden_inherent_c_becomes_rust() {}
}

pub struct DocHiddenInherentMethod;

impl DocHiddenInherentMethod {
    // shouldn't trigger the lint due to #[doc(hidden)] method
    #[doc(hidden)]
    pub extern "C" fn doc_hidden_inherent_c_becomes_rust() {}
}

#[doc(hidden)]
pub trait DocHiddenTraitWithAbi {
    // shouldn't trigger the lint due to #[doc(hidden)] type
    extern "C" fn doc_hidden_trait_c_becomes_rust();
}

pub trait DocHiddenMethodInTraitWithAbi {
    // shouldn't trigger the lint due to #[doc(hidden)] method
    #[doc(hidden)]
    extern "C" fn doc_hidden_trait_c_becomes_rust();
}
