mod sealed {
    pub(crate) trait Sealed {}
}

pub trait WillGainConstWithoutDefault {
    fn make_me_non_object_safe() -> Self;
}

pub trait WillGainConstWithoutDefaultSealed: sealed::Sealed {
    fn make_me_non_object_safe() -> Self;
}
pub trait WillGainConstWithoutDefaultAndSeal {
    fn make_me_non_object_safe() -> Self;
}

pub trait WillGainConstWithDefault {
    fn make_me_non_object_safe() -> Self;
}
pub trait WillGainConstWithDefaultSealed: sealed::Sealed {
    fn make_me_non_object_safe() -> Self;
}

pub trait WillGainAnotherConstWithoutDefault {
    const ONE: bool;

    fn make_me_non_object_safe() -> Self;
}
pub trait WillGainAnotherConstWithoutDefaultSealed: sealed::Sealed {
    const ONE: bool;

    fn make_me_non_object_safe() -> Self;
}

pub trait WillGainADocHiddenConst {
    fn make_me_non_object_safe() -> Self;
}

pub trait ConstWithoutDefaultUnchanged {
    const BAR: bool;

    fn make_me_non_object_safe() -> Self;
}
pub trait ConstDocHidden {
    #[doc(hidden)]
    const BAR: bool = true;

    fn make_me_non_object_safe() -> Self;
}
