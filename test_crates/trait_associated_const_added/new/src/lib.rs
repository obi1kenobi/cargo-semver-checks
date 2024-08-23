mod sealed {
    pub(crate) trait Sealed {}
}

// trigger
pub trait WillGainConstWithoutDefault {
    const BAR: bool;
}

pub trait WillGainConstWithoutDefaultSealed: sealed::Sealed {
    const BAR: bool;
}
// trigger
pub trait WillGainConstWithoutDefaultAndSeal: sealed::Sealed {
    const BAR: bool;
}

pub trait WillGainConstWithDefault {
    const BAR: bool = true;
}
pub trait WillGainConstWithDefaultSealed: sealed::Sealed {
    const BAR: bool = true;
}

// trigger
pub trait WillGainAnotherConstWithoutDefault {
    const ONE: bool;
    const TWO: bool;
}
// trigger
pub trait WillGainAnotherConstWithoutDefaultSealed: sealed::Sealed {
    const ONE: bool;
    const TWO: bool;
}
