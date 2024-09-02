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

pub trait WillGainMethodWithoutDefaultAndSeal: sealed::Sealed {
    fn one_method(self);
} 

pub trait WIllGainDocHiddenMethodWithoutDefault {
    #[doc(hidden)]
    fn one_method(self);
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

pub trait WillKeepAMethodWithoutDefault {
    fn one_method(self);
}
