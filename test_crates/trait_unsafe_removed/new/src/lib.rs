// Public unsafe trait becomes safe, should get reported.
pub trait TraitBecomesSafe {}

// Trait that was not publicly-visible becomes safe, shouldn't get reported.
pub trait TraitBecomesPublicAndSafe {}

// Private trait becomes safe, shouldn't get reported.
trait PrivateTraitBecomesSafe {}
