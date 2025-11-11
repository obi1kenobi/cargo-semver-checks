#![no_std]

pub extern "C" fn unwind_function_becomes_non_unwind() {}

pub extern "C-unwind" fn non_unwind_function_becomes_unwind() {}

pub struct InherentUnwindToggles;

impl InherentUnwindToggles {
    pub extern "C" fn inherent_unwind_becomes_non_unwind() {}
    pub extern "C-unwind" fn inherent_non_unwind_becomes_unwind() {}

    #[doc(hidden)]
    pub extern "C" fn doc_hidden_inherent_method_becomes_non_unwind() {}
}

#[doc(hidden)]
pub struct DocHiddenInherentOwner;

impl DocHiddenInherentOwner {
    pub extern "C" fn doc_hidden_inherent_owner_method_becomes_non_unwind() {}
}

pub trait TraitUnwindToggles {
    extern "C" fn trait_unwind_becomes_non_unwind();
    extern "C-unwind" fn trait_non_unwind_becomes_unwind();

    #[doc(hidden)]
    extern "C" fn doc_hidden_trait_method_becomes_non_unwind();
}

#[doc(hidden)]
pub trait DocHiddenTraitUnwind {
    extern "C" fn doc_hidden_trait_becomes_non_unwind();
}
