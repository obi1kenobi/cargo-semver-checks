mod sealed {
    pub(crate) trait Sealed {}
}

pub trait WillLoseDefault {
    const ONE: bool;
    fn make_me_non_object_safe() -> Self;
}

pub trait WillLoseDefaultSealed: sealed::Sealed {
    const ONE: bool;

    fn make_me_non_object_safe() -> Self;
}
// pub trait WillLostDefaultAndGainSeal {
//     fn make_me_non_object_safe() -> Self;
// }

pub trait Unchanged {
    const ONE: bool = true;

    fn make_me_non_object_safe() -> Self;
}
pub trait UnchangedSealed: sealed::Sealed {
    const ONE: bool = true;

    fn make_me_non_object_safe() -> Self;
}

pub trait UnchangedNoDefault {
    const ONE: bool;

    fn make_me_non_object_safe() -> Self;
}
pub trait UnchangedNoDefaultSealed: sealed::Sealed {
    const ONE: bool;

    fn make_me_non_object_safe() -> Self;
}

// pub trait WillGainADocHiddenConst {
//     fn make_me_non_object_safe() -> Self;
// }

// pub trait ConstWithoutDefaultUnchanged {
//     const BAR: bool;

//     fn make_me_non_object_safe() -> Self;
// }
// pub trait ConstDocHidden {
//     #[doc(hidden)]
//     const BAR: bool = true;

//     fn make_me_non_object_safe() -> Self;
// }
