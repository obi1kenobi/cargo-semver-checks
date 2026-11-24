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

// Removing the inherent functions leaves no public API replacement for these names.
// Downstream crates cannot import Private or Unexported, and HiddenPath's only path is doc-hidden.
impl Private for NoNamedReplacement {}
impl private::Unexported for NoNamedReplacement {}
impl HiddenPath for NoNamedReplacement {}

// With `use method_variadic_trait_visibility::prelude::*`, callers can still name
// NoNamedReplacement::from_prelude, but it no longer accepts variadic arguments.
// The lint misses this change because Anonymous has no named public API path.
impl private::Anonymous for NoNamedReplacement {}

pub struct PublicReplacement;

// Private cannot preserve the old signatures of private_variadic and private_fixed.
// Report each change on Replacement once even though callers can import it by either name.
impl Private for PublicReplacement {}
impl Replacement for PublicReplacement {}

// Through Replacement, from_private_module becomes variadic and from_hidden_path
// stops being variadic. The lints miss both changes because they cannot tell whether
// either trait below was re-exported as `_` and could preserve the old signature.
impl private::Unexported for PublicReplacement {}
impl HiddenPath for PublicReplacement {}

// With `use method_variadic_trait_visibility::prelude::*`, the old fixed-arity
// PublicReplacement::from_prelude pointer still works through Anonymous.
impl private::Anonymous for PublicReplacement {}

pub trait VariadicDefault {
    unsafe extern "C" fn default(_: ...) {}
}

#[repr(transparent)]
pub struct PreludeTraits(u8);
impl VariadicDefault for PreludeTraits {}

// Report both changes to fixed arity. Callers can import Clone and AsRef from core.
// The adapter supplies a declaration for Clone but has none for AsRef.
impl Clone for PreludeTraits {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl AsRef<u8> for PreludeTraits {
    fn as_ref(&self) -> &u8 {
        &self.0
    }
}

// The old `fn() -> PreludeTraits` pointer still works through Default in the Rust prelude,
// even when rustdoc omits the trait declaration and VariadicDefault offers a variadic default.
impl Default for PreludeTraits {
    fn default() -> Self {
        Self(0)
    }
}
