#![no_std]

// Basic test case
pub trait PubTraitA {
    #[doc(hidden)]
    type TypeA;
    // Should not flag already #[doc(hidden)] type
    #[doc(hidden)]
    type DocHiddenType;
}

// Trait is private so it should not trigger the lint
trait TraitA {
    #[doc(hidden)]
    type TypeB;
}

// Trait is #[doc(hidden)] along with the type, only trait_now_doc_hidden should be triggered
#[doc(hidden)]
pub trait PubTraitB {
    #[doc(hidden)]
    type TypeC;
}

// Test cases when trait is #[doc(hidden)]
#[doc(hidden)]
pub trait DocHiddenTrait {
    #[doc(hidden)]
    type TypeD;
    #[doc(hidden)]
    type TypeE;
}

// Test cases when #[doc(hidden)] from trait is removed
pub trait PubTraitC {
    #[doc(hidden)]
    type TypeF;
    #[doc(hidden)]
    type TypeG;
}
