// Public unsafe trait becomes safe, should get reported.
pub unsafe trait TraitBecomesSafe {}

// Trait that was not publicly-visible becomes safe, shouldn't get reported.
unsafe trait TraitBecomesPublicAndSafe {}

// pub unsafe trait becomes private and safe, shouldn't get reported.
pub unsafe trait TraitBecomesPrivateAndSafe {}

// Private trait becomes safe, shouldn't get reported.
unsafe trait PrivateTraitBecomesSafe {}
