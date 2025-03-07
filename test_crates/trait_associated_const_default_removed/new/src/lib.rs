#![no_std]

mod sealed {
    pub(crate) trait Sealed {}
}

pub trait WillLoseDefault {
    const ONE: bool;
    fn make_me_non_dyn_compatible() -> Self;
}

pub trait WillLoseDefaultSealed: sealed::Sealed {
    const ONE: bool;

    fn make_me_non_dyn_compatible() -> Self;
}

pub trait Unchanged {
    const ONE: bool = true;

    fn make_me_non_dyn_compatible() -> Self;
}
pub trait UnchangedSealed: sealed::Sealed {
    const ONE: bool = true;

    fn make_me_non_dyn_compatible() -> Self;
}

pub trait UnchangedNoDefault {
    const ONE: bool;

    fn make_me_non_dyn_compatible() -> Self;
}
pub trait UnchangedNoDefaultSealed: sealed::Sealed {
    const ONE: bool;

    fn make_me_non_dyn_compatible() -> Self;
}

pub trait PublicAPISealed {
    #[doc(hidden)]
    type Hidden;

    const ONE: bool;

    fn make_me_non_dyn_compatible() -> Self;
}
