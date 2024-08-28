mod sealed {
    pub(crate) trait Sealed {}
}

// ---- Should be reported ----
pub trait WillGainMethodWithoutDefault {}

pub trait WillGainAnotherMethodWithoutDefault {
    fn one_method(self);
}

pub trait WillGainMultipleMethodsWithoutDefault {}

pub trait WillGainMethodWithoutDefaultAndSeal {}

pub trait WIllGainDocHiddenMethodWithoutDefault {}

// ---- Should not be reported ----
pub trait WillGainMethodWithDefault {}

pub trait WillGainAnotherMethodWithDefault {
    fn one_method(self);
}

pub trait WillGainMethodWithoutDefaultSealed: sealed::Sealed {}

pub trait WillGainMethodWithoutDefaultAndLoseSeal: sealed::Sealed {}

pub trait WillKeepAMethodWithoutDefault {
    fn one_method(self);
}
