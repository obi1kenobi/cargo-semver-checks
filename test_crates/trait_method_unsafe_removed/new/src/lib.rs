use std::convert::AsRef;

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

// Method of sealed trait becomes safe, shouldn't get reported.
pub trait PrivModSealedTrait: private::Sealed {
    fn becomes_safe();
}

// Method of sealed trait becomes safe, shouldn't get reported.
#[allow(private_bounds)]
pub trait PrivTraitSealedTrait: Sealed {
    fn becomes_safe();
}

// This trait has a supertrait, but it isn't a sealing supertrait, so it should
// get reported.
pub trait UnsealedTrait: Unsealed {
    fn becomes_safe();
}

// This trait has multiple supertraits, one of which is a sealing supertrait, so
// it shouldn't get reported.
pub trait SealedAndUnsealedTrait: private::Sealed + Unsealed {
    fn becomes_safe();
}

// This trait has a supertrait, but it isn't a sealing supertrait, so it should
// get reported.
pub trait TraitWithStdSupertrait: AsRef<()> {
    fn becomes_safe();
}

// Method of sealed trait becomes safe, shouldn't get reported.
pub trait SealedTraitWithStdSupertrait: AsRef<()> + private::Sealed {
    fn becomes_safe();
}

mod private {
    pub trait Sealed {}
}

trait Sealed {}

pub trait Unsealed {}
