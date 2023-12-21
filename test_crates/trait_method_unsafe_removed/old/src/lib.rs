// Method of public trait becomes safe, should get reported.
pub trait PubTrait {
    unsafe fn becomes_safe();
    fn stays_safe();
    unsafe fn stays_unsafe();
}

// Method of trait that was not publicly-visible becomes safe, shouldn't get
// reported.
trait TraitBecomesPublic {
    unsafe fn becomes_safe();
}

// Method of trait that is publicly-visible becomes safe while trait becomes
// private. The trait becoming private should be the only violation that's
// reported.
pub trait TraitBecomesPrivate {
    unsafe fn becomes_safe();
}

// Method of trait becomes safe while trait also becomes safe. Method should get
// reported along with trait.
pub unsafe trait TraitBecomesSafe {
    unsafe fn becomes_safe();
}

// Private trait's method becomes safe, shouldn't get reported.
trait PrivateTrait {
    unsafe fn becomes_safe();
}
