// Method of public trait becomes safe, should get reported.
pub trait PubTrait {
    fn becomes_safe();
    fn stays_safe();
    unsafe fn stays_unsafe();
}

// Method of trait that was not publicly-visible becomes safe, shouldn't get
// reported.
pub trait TraitBecomesPublic {
    fn becomes_safe();
}

// Method of trait that is publicly-visible becomes safe while trait becomes
// private. The trait becoming private should be the only violation that's
// reported.
trait TraitBecomesPrivate {
    fn becomes_safe();
}

// Method of trait becomes safe while trait also becomes safe. Method should get
// reported along with trait.
pub trait TraitBecomesSafe {
    fn becomes_safe();
}

// Private trait's method becomes safe, shouldn't get reported.
trait PrivateTrait {
    fn becomes_safe();
}
