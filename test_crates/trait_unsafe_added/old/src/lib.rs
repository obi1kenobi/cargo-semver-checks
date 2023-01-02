// Public trait becomes unsafe, should get reported.
pub trait TraitBecomesUnsafe {}

// Trait that was not publicly-visible becomes unsafe, shouldn't get reported.
trait TraitBecomesPublicAndUnsafe {}

// Trait that is publicly-visibly becomes private and unsafe, shouldn't get reported.
pub trait TraitBecomesPrivateAndUnsafe {}

// Private trait becomes unsafe, shouldn't get reported.
trait PrivateTraitBecomesUnsafe {}

// Unsafe trait, doesn't get changed.
pub unsafe trait UnsafeTrait {}

// Normal trait, doesn't get changed.
pub trait NormalTrait {}
