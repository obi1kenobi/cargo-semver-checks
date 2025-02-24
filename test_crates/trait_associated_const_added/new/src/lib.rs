mod sealed {
    pub(crate) trait Sealed {}
}

pub trait WillGainConstWithoutDefault {
    const BAR: bool;

    fn make_me_non_dyn_compatible() -> Self;
}

pub trait WillGainConstWithoutDefaultSealed: sealed::Sealed {
    const BAR: bool;

    fn make_me_non_dyn_compatible() -> Self;
}
pub trait WillGainConstWithoutDefaultAndSeal: sealed::Sealed {
    const BAR: bool;

    fn make_me_non_dyn_compatible() -> Self;
}

pub trait WillGainConstWithDefault {
    const BAR: bool = true;

    fn make_me_non_dyn_compatible() -> Self;
}
pub trait WillGainConstWithDefaultSealed: sealed::Sealed {
    const BAR: bool = true;

    fn make_me_non_dyn_compatible() -> Self;
}

pub trait WillGainAnotherConstWithoutDefault {
    const ONE: bool;
    const TWO: bool;

    fn make_me_non_dyn_compatible() -> Self;
}
pub trait WillGainAnotherConstWithoutDefaultSealed: sealed::Sealed {
    const ONE: bool;
    const TWO: bool;

    fn make_me_non_dyn_compatible() -> Self;
}

pub trait WillGainADocHiddenConst {
    #[doc(hidden)]
    const BAR: bool;

    fn make_me_non_dyn_compatible() -> Self;
}

pub trait ConstWithoutDefaultUnchanged {
    const BAR: bool;

    fn make_me_non_dyn_compatible() -> Self;
}
pub trait ConstDocHidden {
    #[doc(hidden)]
    const BAR: bool = true;

    fn make_me_non_dyn_compatible() -> Self;
}

pub trait PublicAPISealed {
    #[doc(hidden)]
    type Hidden;

    const BAR: bool = true;

    fn make_me_non_dyn_compatible() -> Self;
}
