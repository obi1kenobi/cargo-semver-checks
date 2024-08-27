use std::convert::AsRef;

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

// Method of sealed trait becomes safe, shouldn't get reported.
pub trait PrivModSealedTrait: private::Sealed {
    unsafe fn becomes_safe();
}

// Method of sealed trait becomes safe, shouldn't get reported.
#[allow(private_bounds)]
pub trait PrivTraitSealedTrait: Sealed {
    unsafe fn becomes_safe();
}

// This trait has a supertrait, but it isn't a sealing supertrait, so it should
// get reported.
pub trait UnsealedTrait: Unsealed {
    unsafe fn becomes_safe();
}

// This trait has multiple supertraits, one of which is a sealing supertrait, so
// it shouldn't get reported.
pub trait SealedAndUnsealedTrait: private::Sealed + Unsealed {
    unsafe fn becomes_safe();
}

// This trait has a supertrait, but it isn't a sealing supertrait, so it should
// get reported.
pub trait TraitWithStdSupertrait: AsRef<()> {
    unsafe fn becomes_safe();
}

// Method of sealed trait becomes safe, shouldn't get reported.
pub trait SealedTraitWithStdSupertrait: AsRef<()> + private::Sealed {
    unsafe fn becomes_safe();
}

mod private {
    pub trait Sealed {}
}

trait Sealed {}

pub trait Unsealed {}
