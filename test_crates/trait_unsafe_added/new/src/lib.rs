// Public trait becomes unsafe, should get reported.
pub unsafe trait TraitBecomesUnsafe {}

// Trait that was not publicly-visible becomes unsafe, shouldn't get reported.
pub unsafe trait TraitBecomesPublicAndUnsafe {}

// Private trait becomes unsafe, shouldn't get reported.
unsafe trait PrivateTraitBecomesUnsafe {}

// Unsafe trait, doesn't get changed.
pub unsafe trait UnsafeTrait {}

// Normal trait, doesn't get changed.
pub trait NormalTrait {}
