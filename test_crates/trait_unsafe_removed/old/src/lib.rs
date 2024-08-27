// Public unsafe trait becomes safe, should get reported.
pub unsafe trait TraitBecomesSafe {}

// Trait that was not publicly-visible becomes safe, shouldn't get reported.
unsafe trait TraitBecomesPublicAndSafe {}

// Trait becomes private and safe. The fact that it's private dominates,
// and should be the only violation that's reported:
// foreign `impl` blocks as a whole are illegal, so it's not specifically
// the `unsafe impl` bit being illegal that's the problem.
pub unsafe trait TraitBecomesPrivateAndSafe {}

// Private trait becomes safe, shouldn't get reported.
unsafe trait PrivateTraitBecomesSafe {}

mod private {
    pub trait Sealed {}
}

// Sealed trait, becoming safe doesn't matter since it cannot be implemented downstream.
pub unsafe trait SealedTrait: private::Sealed {}
