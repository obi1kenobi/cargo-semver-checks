mod sealed {
    pub(crate) trait Sealed {}
}

pub trait WillGainTypeWithoutDefault {
    type Bar;

    fn make_me_non_object_safe() -> Self;
}

pub trait WillGainTypeWithoutDefaultSealed: sealed::Sealed {
    type BAR;

    fn make_me_non_object_safe() -> Self;
}
pub trait WillGainTypeWithoutDefaultAndSeal: sealed::Sealed {
    type BAR;

    fn make_me_non_object_safe() -> Self;
}

pub trait WillGainTypeWithDefault {
    type BAR = bool;

    fn make_me_non_object_safe() -> Self;
}
pub trait WillGainTypeWithDefaultSealed: sealed::Sealed {
    type BAR = bool;

    fn make_me_non_object_safe() -> Self;
}

pub trait WillGainAnotherTypeWithoutDefault {
    type ONE;
    type TWO;

    fn make_me_non_object_safe() -> Self;
}
pub trait WillGainAnotherTypeWithoutDefaultSealed: sealed::Sealed {
    type ONE;
    type TWO;

    fn make_me_non_object_safe() -> Self;
}

pub trait WillGainADocHiddenType {
    #[doc(hidden)]
    type BAR;

    fn make_me_non_object_safe() -> Self;
}

pub trait TypeWithoutDefaultUnchanged {
    type BAR;

    fn make_me_non_object_safe() -> Self;
}
pub trait TypeDocHidden {
    #[doc(hidden)]
    type BAR;

    fn make_me_non_object_safe() -> Self;
}