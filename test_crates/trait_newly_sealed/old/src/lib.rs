// Public trait becomes sealed, should be reported
pub trait TraitBecomesSealed {}

// Trait that was not publicly-visible becomes sealed, shouldnt get reported as its still private
trait TraitRemainsPrivateButSealed {}

// Method on public trait becomes sealed, should be reported
pub trait TraitMethodBecomesSealed {
    fn method(&self);
}
// Trait that was not publicly-visible becomes public and sealed, shouldnt be
// reported
trait TraitBecomesPublicAndSealed {}

// Trait becomes private and sealed. The fact that it's private dominates,
// and should be the only violation that's reported:
// Thus being newly sealed is not the main problem
pub trait TraitBecomesPrivateAndSealed {}
