// Public trait becomes unsafe, should get reported.
pub trait TraitBecomesUnsafe {}

// Private trait becomes unsafe, shouldn't get reported.
trait PrivateTraitBecomesUnsafe {}

// Unsafe trait, doesn't get changed.
pub unsafe trait UnsafeTrait {}

// Normal trait, doesn't get changed.
pub trait NormalTrait {}
