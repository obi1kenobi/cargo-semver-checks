#![no_std]

pub enum VariantNowDocHidden {
    Visible,
    /// Testing: <https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html#hidden>
    WillBeHidden,
}

pub enum VariantAlreadyHidden {
    Visible,
    #[doc(hidden)]
    AlreadyHidden,
}

mod private_mod {
    pub enum PrivateEnum {
        HiddenLater,
    }
}
