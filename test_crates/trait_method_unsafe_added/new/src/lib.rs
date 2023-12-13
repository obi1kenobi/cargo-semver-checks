// Method of public trait becomes unsafe, should get reported.
pub trait PubTrait {
    unsafe fn becomes_unsafe();
    fn stays_safe();
    unsafe fn stays_unsafe();
}

// Method of trait that was not publicly-visible becomes unsafe, shouldn't get
// reported.
pub trait TraitBecomesPublic {
    unsafe fn becomes_unsafe();
}

// Method of trait that is publicly-visible becomes unsafe while trait becomes
// private. The trait becoming private should be the only violation that's
// reported.
trait TraitBecomesPrivate {
    unsafe fn becomes_unsafe();
}

// Method of trait becomes unsafe while trait also becomes unsafe. Method should
// get reported along with trait.
pub unsafe trait TraitBecomesUnsafe {
    unsafe fn becomes_unsafe();
}

// Private trait's method becomes unsafe, shouldn't get reported.
trait PrivateTrait {
    unsafe fn becomes_unsafe();
}
