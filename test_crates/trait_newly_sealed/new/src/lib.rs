mod private {
    pub trait Sealed {}
    pub struct Token;
}

// Public trait becomes sealed, should be reported
pub trait TraitBecomesSealed: private::Sealed {}

// Trait that was not publicly-visible becomes sealed, shouldnt get reported as its still private
trait TraitRemainsPrivateButSealed: private::Sealed {}

// Method on public trait becomes sealed, should be reported
pub trait TraitMethodBecomesSealed {
    fn method(&self, _: private::Token);
}

// Trait that was not publicly-visible becomes public and sealed, shouldnt be
// reported
pub trait TraitBecomesPublicAndSealed: private::Sealed {}

// Trait becomes private and sealed. The fact that it's private dominates,
// and should be the only violation that's reported:
// Thus being newly sealed is not the main problem
trait TraitBecomesPrivateAndSealed: private::Sealed {}
