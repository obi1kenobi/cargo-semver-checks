#![no_std]

pub fn existing_function() {}

pub mod my_module {
    // This should trigger this lint.
    pub fn new_function_in_module() {}
}

pub struct ExistingStruct;

impl ExistingStruct {
    // Inherent methods are reported by `inherent_method_added`, not this lint.
    pub fn new_method() {}
}

// This should trigger this lint.
pub fn new_function() {}

// Private function, this should not trigger this lint.
fn new_private_function() {}

// Not public API, this should not trigger this lint.
#[doc(hidden)]
pub fn new_doc_hidden_function() {}

mod private_module {
    // Not importable from outside the crate, this should not trigger this lint.
    pub fn new_unreachable_function() {}

    // Re-exported at the crate root below, this should trigger this lint.
    pub fn new_reexported_function() {}
}

pub use private_module::new_reexported_function;

pub mod reexports {
    // An existing function is now importable at a new path.
    // This should trigger this lint.
    pub use crate::existing_function;
}
