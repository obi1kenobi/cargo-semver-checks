// Public unsafe trait becomes safe, should get reported.
pub trait TraitBecomesSafe {}

// Trait that was not publicly-visible becomes safe, shouldn't get reported.
pub trait TraitBecomesPublicAndSafe {}

// Trait becomes private and safe. The fact that it's private dominates,
// and should be the only violation that's reported:
// foreign `impl` blocks as a whole are illegal, so it's not specifically
// the `unsafe impl` bit being illegal that's the problem.
trait TraitBecomesPrivateAndSafe {}

// Private trait becomes safe, shouldn't get reported.
trait PrivateTraitBecomesSafe {}

mod private {
    pub trait Sealed {}
}

// Sealed trait, becoming safe doesn't matter since it cannot be implemented downstream.
pub trait SealedTrait: private::Sealed {}
