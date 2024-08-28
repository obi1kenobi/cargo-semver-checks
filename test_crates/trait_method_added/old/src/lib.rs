mod sealed {
    pub(crate) trait Sealed {}
}

// ---- Should be reported ----
pub trait WillGainMethodWithoutDefault {}

pub trait WillGainAnotherMethodWithoutDefault {
    fn one_method(self);
}

pub trait WillGainMultipleMethodsWithoutDefault {}

// ---- Should not be reported ----
pub trait WillGainMethodWithDefault {}

pub trait WillGainAnotherMethodWithDefault {
    fn one_method(self);
}

pub trait WillGainMethodWithoutDefaultSealed: sealed::Sealed {}

pub trait WillGainMethodWithoutDefaultAndLoseSeal: sealed::Sealed {}

/*
Will let this case to be reported only by the newly sealed trait Lint,
and not by this one, since sealing a trait indicates that the user 
wants to remove this type from the public API.
*/
pub trait WillGainMethodWithoutDefaultAndSeal {} 
