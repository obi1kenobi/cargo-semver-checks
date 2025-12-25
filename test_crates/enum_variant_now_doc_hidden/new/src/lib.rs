#![no_std]

pub enum VariantNowDocHidden {
    Visible,
    #[doc(hidden)]
    WillBeHidden,
}

pub enum VariantAlreadyHidden {
    Visible,
    #[doc(hidden)]
    AlreadyHidden,
}

mod private_mod {
    pub enum PrivateEnum {
        #[doc(hidden)]
        HiddenLater,
    }
}
