#![no_std]

// Basic test case
pub trait PubTraitA {
    #[doc(hidden)]
    const CONST_A: u8 = 0;
    // Should not flag already #[doc(hidden)] const
    #[doc(hidden)]
    const DOC_HIDDEN_CONST_A: u8 = 0;
}

// Trait is private so it should not trigger the lint
trait TraitA {
    #[doc(hidden)]
    const CONST_B: u8 = 0;
}

// Trait is #[doc(hidden)] along with the const, only trait_now_doc_hidden should be triggered
#[doc(hidden)]
pub trait PubTraitB {
    #[doc(hidden)]
    const CONST_C: u8 = 0;
}

// Test cases when trait is #[doc(hidden)]
#[doc(hidden)]
pub trait DocHiddenTrait {
    #[doc(hidden)]
    const CONST_D: u8 = 0;
    #[doc(hidden)]
    const DOC_HIDDEN_CONST_B: u8 = 0;
}

// Test cases when #[doc(hidden)] from trait is removed
pub trait PubTraitC {
    #[doc(hidden)]
    const CONST_E: u8 = 0;
    #[doc(hidden)]
    const DOC_HIDDEN_CONST_C: u8 = 0;
}
