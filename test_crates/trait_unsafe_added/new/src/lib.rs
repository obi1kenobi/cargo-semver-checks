// Public trait becomes unsafe, should get reported.
pub unsafe trait TraitBecomesUnsafe {}

// Trait that was not publicly-visible becomes unsafe, shouldn't get reported.
pub unsafe trait TraitBecomesPublicAndUnsafe {}

// Trait that is publicly-visibly becomes private and unsafe.
// The fact that it's private dominates, and should be the only violation that's reported:
// foreign `impl` blocks as a whole are illegal, so it's not specifically
// the `unsafe impl` bit being required that's the problem.
unsafe trait TraitBecomesPrivateAndUnsafe {}

// Private trait becomes unsafe, shouldn't get reported.
unsafe trait PrivateTraitBecomesUnsafe {}

// Unsafe trait, doesn't get changed.
pub unsafe trait UnsafeTrait {}

// Normal trait, doesn't get changed.
pub trait NormalTrait {}

mod private {
    pub trait Sealed {}
}

// Sealed trait, becoming unsafe doesn't matter since it cannot be implemented downstream.
pub unsafe trait SealedTrait: private::Sealed {}
