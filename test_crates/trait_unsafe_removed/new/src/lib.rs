// Public unsafe trait becomes safe, should get reported.
pub trait TraitBecomesSafe {}

// Trait that was not publicly-visible becomes safe, shouldn't get reported.
pub trait TraitBecomesPublicAndSafe {}

// pub unsafe trait becomes private and safe, shouldn't get reported.
trait TraitBecomesPrivateAndSafe {}

// Private trait becomes safe, shouldn't get reported.
trait PrivateTraitBecomesSafe {}
