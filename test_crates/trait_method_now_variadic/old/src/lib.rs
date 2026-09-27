#![no_std]
// Rustdoc fixtures also run on Rust versions predating
// stabilization of C-variadic definitions in Rust 1.99.
// TODO: Remove the feature gate and stable_features allowance when Rust 1.99 is the MSRV.
#![feature(c_variadic)]
#![allow(stable_features)]

// Sized keeps these traits non-dyn-compatible on both sides of the change.
// All existing methods in this trait should be reported.
pub trait RequiredAndProvided: Sized {
    unsafe extern "C" fn required_associated();
    unsafe extern "C" fn provided_associated(_value: i32) {}
    unsafe extern "C" fn required_method(&self);
    unsafe extern "C" fn provided_method(&mut self, _value: i32) {}
    unsafe extern "C-unwind" fn c_unwind(_value: i32);
}

// Sealing does not prevent downstream function-pointer coercions.
// Both methods should be reported.
pub trait Sealed: private::Sealed + Sized {
    unsafe extern "C" fn required_associated();
    unsafe extern "C" fn provided_method(&self) {}
}

mod private {
    pub trait Sealed {}

    // The public reexport makes this method part of the public API.
    pub trait Reexported: Sized {
        unsafe extern "C" fn method(&self);
    }

    // An unreachable public trait should not be reported.
    pub trait Unreachable: Sized {
        unsafe extern "C" fn method(&self);
    }
}

pub use private::Reexported as PublicReexport;

// Private traits should not be reported.
trait Private: Sized {
    unsafe extern "C" fn method(&self);
}

// Hidden traits and methods should not be reported.
#[doc(hidden)]
pub trait Hidden: Sized {
    unsafe extern "C" fn method(&self);
}

pub trait WithHiddenMethod: Sized {
    #[doc(hidden)]
    unsafe extern "C" fn method(&self);
}

// These methods were not previously public API, so should not be reported.
trait NewlyPublic: Sized {
    unsafe extern "C" fn method(&self);
}

#[doc(hidden)]
pub trait NewlyVisible: Sized {
    unsafe extern "C" fn method(&self);
}

pub trait WithNewlyVisibleMethod: Sized {
    #[doc(hidden)]
    unsafe extern "C" fn method(&self);
}
