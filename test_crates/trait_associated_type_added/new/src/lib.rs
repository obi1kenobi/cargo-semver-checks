#![no_std]

#![feature(associated_type_defaults)]

mod sealed {
    pub(crate) trait Sealed {}
}

pub trait WillGainTypeWithoutDefault {
    type Bar;

    fn make_me_non_dyn_compatible() -> Self;
}

pub trait WillGainTypeWithoutDefaultSealed: sealed::Sealed {
    type Bar;

    fn make_me_non_dyn_compatible() -> Self;
}

pub trait WillGainTypeWithoutDefaultAndSeal: sealed::Sealed {
    type Bar;

    fn make_me_non_dyn_compatible() -> Self;
}

pub trait WillGainTypeWithDefault {
    type Bar = bool;

    fn make_me_non_dyn_compatible() -> Self;
}

pub trait WillGainTypeWithDefaultSealed: sealed::Sealed {
    type Bar = bool;

    fn make_me_non_dyn_compatible() -> Self;
}

pub trait WillGainAnotherTypeWithoutDefault {
    type One;
    type Two;

    fn make_me_non_dyn_compatible() -> Self;
}

pub trait WillGainAnotherTypeWithoutDefaultSealed: sealed::Sealed {
    type One;
    type Two;

    fn make_me_non_dyn_compatible() -> Self;
}

pub trait WillGainADocHiddenType {
    #[doc(hidden)]
    type Bar;

    fn make_me_non_dyn_compatible() -> Self;
}

pub trait TypeWithoutDefaultUnchanged {
    type Bar;

    fn make_me_non_dyn_compatible() -> Self;
}

pub trait TypeDocHidden {
    #[doc(hidden)]
    type Bar;

    fn make_me_non_dyn_compatible() -> Self;
}

pub trait PublicAPISealed {
    #[doc(hidden)]
    type Hidden;

    type Bar;

    fn make_me_non_dyn_compatible() -> Self;
}
