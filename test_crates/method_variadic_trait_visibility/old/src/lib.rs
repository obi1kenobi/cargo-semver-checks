#![no_std]
// Rustdoc fixtures also run on Rust versions predating
// stabilization of C-variadic definitions in Rust 1.99.
// TODO: Remove the feature gate and stable_features allowance when Rust 1.99 is the MSRV.
#![feature(c_variadic)]
#![allow(stable_features)]

trait Private {
    unsafe extern "C" fn private_variadic(_: i32, _: ...) {}
    unsafe extern "C" fn private_fixed(_: i32) {}
}

mod private {
    pub trait Unexported {
        unsafe extern "C" fn from_private_module(_: i32) {}
    }

    pub trait HiddenPath {
        unsafe extern "C" fn from_hidden_path(_: i32, _: ...) {}
        unsafe extern "C" fn fixed_from_hidden_path(_: i32) {}
    }

    pub trait Anonymous {
        unsafe extern "C" fn from_prelude(_: i32) {}
    }

    pub trait Replacement {
        unsafe extern "C" fn private_variadic(_: i32) {}
        unsafe extern "C" fn private_fixed(_: i32, _: ...) {}
        unsafe extern "C" fn from_private_module(_: i32, _: ...) {}
        unsafe extern "C" fn from_hidden_path(_: i32) {}
        unsafe extern "C" fn from_prelude(_: i32, _: ...) {}
    }
}

pub mod prelude {
    pub use crate::private::Anonymous as _;
}

#[doc(hidden)]
pub use private::HiddenPath;
pub use private::Replacement;
pub use private::Replacement as ReplacementAlias;

pub struct NoNamedReplacement;
impl Private for NoNamedReplacement {}
impl private::Unexported for NoNamedReplacement {}
impl HiddenPath for NoNamedReplacement {}
impl private::Anonymous for NoNamedReplacement {}
impl NoNamedReplacement {
    pub unsafe extern "C" fn private_variadic(_: i32) {}
    pub unsafe extern "C" fn private_fixed(_: i32, _: ...) {}
    pub unsafe extern "C" fn from_private_module(_: i32, _: ...) {}
    pub unsafe extern "C" fn from_hidden_path(_: i32) {}
    pub unsafe extern "C" fn fixed_from_hidden_path(_: i32, _: ...) {}
    pub unsafe extern "C" fn from_prelude(_: i32, _: ...) {}
}

pub struct PublicReplacement;
impl Private for PublicReplacement {}
impl Replacement for PublicReplacement {}
impl private::Unexported for PublicReplacement {}
impl HiddenPath for PublicReplacement {}
impl private::Anonymous for PublicReplacement {}
impl PublicReplacement {
    pub unsafe extern "C" fn private_variadic(_: i32, _: ...) {}
    pub unsafe extern "C" fn private_fixed(_: i32) {}
    pub unsafe extern "C" fn from_private_module(_: i32) {}
    pub unsafe extern "C" fn from_hidden_path(_: i32, _: ...) {}
    pub unsafe extern "C" fn from_prelude(_: i32) {}
}

pub trait VariadicDefault {
    unsafe extern "C" fn default(_: ...) {}
}

#[repr(transparent)]
pub struct PreludeTraits(u8);
impl VariadicDefault for PreludeTraits {}

impl PreludeTraits {
    pub unsafe extern "C" fn clone(&self, _: ...) -> Self {
        Self(self.0)
    }

    pub unsafe extern "C" fn as_ref(&self, _: ...) -> &u8 {
        &self.0
    }

    pub fn default() -> Self {
        Self(0)
    }
}
