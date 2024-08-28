mod sealed {
    pub(crate) trait Sealed {}
}

// ---- Should be reported ----
pub trait WillGainMethodWithoutDefault {
    fn one_method(self);
}

pub trait WillGainAnotherMethodWithoutDefault {
    fn one_method(self);
    fn two_method(self);
}

pub trait WillGainMultipleMethodsWithoutDefault {
    fn one_method(self);
    fn two_method(self);
}

// ---- Should not be reported ----
pub trait WillGainMethodWithDefault {
    fn one_method(self) {}
}

pub trait WillGainAnotherMethodWithDefault {
    fn one_method(self);
    fn two_method(self) {}
}

pub trait WillGainMethodWithoutDefaultSealed: sealed::Sealed {
    fn one_method(self);
}

pub trait WillGainMethodWithoutDefaultAndLoseSeal {
    fn one_method(self);
}

/*
Will let this case to be reported only by the newly sealed trait Lint,
and not by this one, since sealing a trait indicates that the user 
wants to remove this type from the public API.
*/
pub trait WillGainMethodWithoutDefaultAndSeal: sealed::Sealed {
    fn one_method(self);
} 
