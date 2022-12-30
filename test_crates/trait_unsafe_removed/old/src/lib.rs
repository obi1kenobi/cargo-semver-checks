// Public unsafe trait becomes safe, should get reported.
pub unsafe trait TraitBecomesSafe {}

// Trait that was not publicly-visible becomes safe, shouldn't get reported.
unsafe trait TraitBecomesPublicAndSafe {}

// Private trait becomes safe, shouldn't get reported.
unsafe trait PrivateTraitBecomesSafe {}
