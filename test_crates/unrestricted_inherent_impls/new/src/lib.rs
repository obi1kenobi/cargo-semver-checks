#![no_std]

extern crate alloc;
use alloc::boxed::Box;

pub struct Reader<T>(pub T);

// Both releases define these methods and constants for every instantiation of Reader.
impl<U> Reader<U> {
    #[doc(hidden)]
    pub const HIDE: u8 = 0;
    #[deprecated]
    pub const DEPRECATE: u8 = 0;

    #[doc(hidden)]
    pub fn hide(&self) {}
    #[deprecated]
    pub fn deprecate(&self) {}
    #[must_use]
    pub fn add_must_use(&self) -> u8 {
        0
    }

    pub fn remove_must_use(&self) -> u8 {
        0
    }

    pub fn drop_value(&self) {}

    pub fn to_mut(&mut self) {}
    pub fn ref_to_owned(self) {}
    pub fn mut_to_owned(self) {}
    pub fn to_box(self: Box<Self>) {}

    // The explicit owner type and Self denote the same receiver in these impls.
    pub fn inspect(self: &Self) {}

    pub extern "Rust" fn change_abi() {}
    pub extern "C-unwind" fn allow_unwind() {}
    pub extern "C" fn forbid_unwind() {}
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl<T> Reader<T> {
    #[target_feature(enable = "avx")]
    pub fn safe_feature_added(&self) {}
    #[target_feature(enable = "avx")]
    pub unsafe fn unsafe_feature_added(&self) {}

    #[target_feature(enable = "avx")]
    #[target_feature(enable = "avx2")]
    pub fn safe_more_features(&self) {}
    #[target_feature(enable = "avx")]
    #[target_feature(enable = "avx2")]
    pub unsafe fn unsafe_more_features(&self) {}
}

// The default preserves Arity<T> in downstream code. Each impl covers every
// instantiation of its owner in its release.
pub struct Arity<T, const N: usize = 0>(T);

impl<T, const N: usize> Arity<T, N> {
    pub const fn value() -> u8 {
        0
    }
}

// A lifetime parameter permits a fixed export name because it does not cause
// separate monomorphizations.
pub struct Exported<'a>(&'a ());

impl<'a> Exported<'a> {
    #[unsafe(export_name = "unrestricted_inherent_impls_new_symbol")]
    pub extern "C" fn symbol() {}
}

// The adapter cannot establish full coverage for the Copy-bound impl in each
// pair. The other release's unrestricted impl is insufficient to report a match.
pub struct Unknown<T>(pub T);

impl<T> Unknown<T> {
    pub fn baseline_bound(self) {}
}

impl<T: Copy> Unknown<T> {
    pub fn current_bound(self) {}
}
