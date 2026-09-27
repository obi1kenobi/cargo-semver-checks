#![no_std]
// Rustdoc fixtures also run on Rust versions predating
// stabilization of C-variadic definitions in Rust 1.99.
// TODO: Remove the feature gate and stable_features allowance when Rust 1.99 is the MSRV.
#![feature(c_variadic)]
#![allow(stable_features)]

// Sized keeps these traits non-dyn-compatible on both sides of the change.
// All existing methods in this trait should be reported.
pub trait RequiredAndProvided: Sized {
    unsafe extern "C" fn required_associated(_: ...);
    unsafe extern "C" fn provided_associated(_value: i32, _: ...) {}
    unsafe extern "C" fn required_method(&self, _: ...);
    unsafe extern "C" fn provided_method(&mut self, _value: i32, _: ...) {}
    unsafe extern "C-unwind" fn c_unwind(_value: i32, _: ...);

    // A new provided method should not be reported.
    unsafe extern "C" fn newly_added(_: ...) {}
}

// Fixed parameter counts and variadic status are independent signature changes.
// Both lints should report these: reverting either change still breaks function pointers.
pub trait FixedParameterCountChanged: Sized {
    unsafe extern "C" fn added_fixed_parameter(_first: i32, _second: i32, _: ...);

    // Removing a fixed parameter while adding `...` preserves existing two-argument calls,
    // but assignments to the previous function pointer type still break.
    unsafe extern "C" fn removed_fixed_parameter(&self, _first: i32, _: ...) {}
}

// Sealing does not prevent downstream function-pointer coercions.
// Both methods should be reported.
pub trait Sealed: private::Sealed + Sized {
    unsafe extern "C" fn required_associated(_: ...);
    unsafe extern "C" fn provided_method(&self, _: ...) {}
}

mod private {
    pub trait Sealed {}

    // The public reexport makes this method part of the public API.
    pub trait Reexported: Sized {
        unsafe extern "C" fn method(&self, _: ...);
    }

    // An unreachable public trait should not be reported.
    pub trait Unreachable: Sized {
        unsafe extern "C" fn method(&self, _: ...);
    }
}

pub use private::Reexported as PublicReexport;

// Private traits should not be reported.
trait Private: Sized {
    unsafe extern "C" fn method(&self, _: ...);
}

// Hidden traits and methods should not be reported.
#[doc(hidden)]
pub trait Hidden: Sized {
    unsafe extern "C" fn method(&self, _: ...);
}

pub trait WithHiddenMethod: Sized {
    #[doc(hidden)]
    unsafe extern "C" fn method(&self, _: ...);
}

// These methods were not previously public API, so should not be reported.
pub trait NewlyPublic: Sized {
    unsafe extern "C" fn method(&self, _: ...);
}

pub trait NewlyVisible: Sized {
    unsafe extern "C" fn method(&self, _: ...);
}

pub trait WithNewlyVisibleMethod: Sized {
    unsafe extern "C" fn method(&self, _: ...);
}
