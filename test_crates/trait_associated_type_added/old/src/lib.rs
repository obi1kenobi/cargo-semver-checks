#![feature(associated_type_defaults)]

mod sealed {
    pub(crate) trait Sealed {}
}

pub trait WillGainTypeWithoutDefault {
    fn make_me_non_object_safe() -> Self;
}

pub trait WillGainTypeWithoutDefaultSealed: sealed::Sealed {
    fn make_me_non_object_safe() -> Self;
}

pub trait WillGainTypeWithoutDefaultAndSeal {
    fn make_me_non_object_safe() -> Self;
}

pub trait WillGainTypeWithDefault {
    fn make_me_non_object_safe() -> Self;
}
pub trait WillGainTypeWithDefaultSealed: sealed::Sealed {
    fn make_me_non_object_safe() -> Self;
}

pub trait WillGainAnotherTypeWithoutDefault {
    type One;

    fn make_me_non_object_safe() -> Self;
}

pub trait WillGainAnotherTypeWithoutDefaultSealed: sealed::Sealed {
    type One;

    fn make_me_non_object_safe() -> Self;
}

pub trait WillGainADocHiddenType {
    fn make_me_non_object_safe() -> Self;
}

pub trait TypeWithoutDefaultUnchanged {
    type Bar;

    fn make_me_non_object_safe() -> Self;
}

pub trait TypeDocHidden {
    #[doc(hidden)]
    type Bar;

    fn make_me_non_object_safe() -> Self;
}
