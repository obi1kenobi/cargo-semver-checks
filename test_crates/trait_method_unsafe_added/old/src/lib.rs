// Method of public trait becomes unsafe, should get reported.
pub trait PubTrait {
    fn becomes_unsafe();
    fn stays_safe();
    unsafe fn stays_unsafe();
}

// Method of trait that was not publicly-visible becomes unsafe, shouldn't get
// reported.
trait TraitBecomesPublic {
    fn becomes_unsafe();
}

// Method of trait that is publicly-visible becomes unsafe while trait becomes
// private. The trait becoming private should be the only violation that's
// reported.
pub trait TraitBecomesPrivate {
    fn becomes_unsafe();
}

// Method of trait becomes unsafe while trait also becomes unsafe. Method should
// get reported along with trait.
pub trait TraitBecomesUnsafe {
    fn becomes_unsafe();
}

// Private trait's method becomes unsafe, shouldn't get reported.
trait PrivateTrait {
    fn becomes_unsafe();
}

// Method of sealed trait becomes unsafe, but should get reported, since calling
// the method now requires an unsafe block.
pub trait SealedTrait: private::Sealed {
    fn becomes_unsafe();
}

mod private {
    pub trait Sealed {}
}
