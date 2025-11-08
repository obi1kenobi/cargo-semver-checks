#![no_std]

#[doc(hidden)]
pub trait SealSuper {}

mod private {
    pub trait Sealed {}
}

// ===== Unsealed traits =====

pub trait UnsealedTrait {
    fn mut_to_ref(&mut self);
    fn owned_to_ref(self);
    fn owned_to_mut(self);
}

#[doc(hidden)]
pub trait DocHiddenUnsealedTrait {
    fn mut_to_ref(&mut self);
    fn owned_to_ref(self);
    fn owned_to_mut(self);
}

pub trait UnsealedTraitWithHiddenMethods {
    #[doc(hidden)]
    fn hidden_mut_to_ref(&mut self);

    #[doc(hidden)]
    fn hidden_owned_to_ref(self);

    #[doc(hidden)]
    fn hidden_owned_to_mut(self);
}

// ===== Public API sealed traits =====

pub trait PublicApiSealedTrait: SealSuper {
    fn mut_to_ref(&mut self);
    fn owned_to_ref(self);
    fn owned_to_mut(self);
}

#[doc(hidden)]
pub trait DocHiddenPublicApiSealedTrait: SealSuper {
    fn mut_to_ref(&mut self);
    fn owned_to_ref(self);
    fn owned_to_mut(self);
}

pub trait PublicApiSealedTraitWithHiddenMethods: SealSuper {
    #[doc(hidden)]
    fn hidden_mut_to_ref(&mut self);

    #[doc(hidden)]
    fn hidden_owned_to_ref(self);

    #[doc(hidden)]
    fn hidden_owned_to_mut(self);
}

// ===== Unconditionally sealed traits =====

pub trait UnconditionallySealedTrait: private::Sealed {
    fn mut_to_ref(&mut self);
    fn owned_to_ref(self);
    fn owned_to_mut(self);
}
