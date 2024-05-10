// Basic test case
pub trait PUB_TRAIT_A {
    type TYPE_A;
    // Should not flag already #[doc(hidden)] type
    #[doc(hidden)]
    type DOC_HIDDEN_TYPE;
}

// Trait is private so it should not trigger the lint
trait TRAIT_A {
    type TYPE_B;
}

// Trait is #[doc(hidden)] along with the type, only trait_now_doc_hidden should be triggered
pub trait PUB_TRAIT_B {
    type TYPE_C;
}

// Test cases when trait is #[doc(hidden)]
#[doc(hidden)]
pub trait DOC_HIDDEN_TRAIT {
    type TYPE_D;
    #[doc(hidden)]
    type TYPE_E;
}

// Test cases when #[doc(hidden)] from trait is removed
#[doc(hidden)]
pub trait PUB_TRAIT_C {
    type TYPE_F;
    #[doc(hidden)]
    type TYPE_G;
}
