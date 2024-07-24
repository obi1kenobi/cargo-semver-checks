// Basic test case
pub trait PubTraitA {
    type TypeA;
    // Should not flag already #[doc(hidden)] type
    #[doc(hidden)]
    type DocHiddenType;
}

// Trait is private so it should not trigger the lint
trait TraitA {
    type TypeB;
}

// Trait is #[doc(hidden)] along with the type, only trait_now_doc_hidden should be triggered
pub trait PubTraitB {
    type TypeC;
}

// Test cases when trait is #[doc(hidden)]
#[doc(hidden)]
pub trait DocHiddenTrait {
    type TypeD;
    #[doc(hidden)]
    type TypeE;
}

// Test cases when #[doc(hidden)] from trait is removed
#[doc(hidden)]
pub trait PubTraitC {
    type TypeF;
    #[doc(hidden)]
    type TypeG;
}
