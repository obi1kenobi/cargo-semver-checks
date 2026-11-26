#![no_std]

pub struct Reader<T>(pub T);

// Both releases define these methods and constants for every instantiation of Reader.
impl<T> Reader<T> {
    pub const HIDE: u8 = 0;
    pub const DEPRECATE: u8 = 0;

    pub fn hide(&self) {}
    pub fn deprecate(&self) {}
    pub fn add_must_use(&self) -> u8 {
        0
    }

    #[must_use]
    pub fn remove_must_use(&self) -> u8 {
        0
    }

    pub fn drop_value(&self) -> u8 {
        0
    }

    pub fn to_mut(&self) {}
    pub fn ref_to_owned(&self) {}
    pub fn mut_to_owned(&mut self) {}
    pub fn to_box(&self) {}

    // The explicit owner type and Self denote the same receiver in these impls.
    pub fn inspect(self: &Reader<T>) {}

    pub extern "C" fn change_abi() {}
    pub extern "C" fn allow_unwind() {}
    pub extern "C-unwind" fn forbid_unwind() {}
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl<T> Reader<T> {
    pub fn safe_feature_added(&self) {}
    pub unsafe fn unsafe_feature_added(&self) {}

    #[target_feature(enable = "avx")]
    pub fn safe_more_features(&self) {}
    #[target_feature(enable = "avx")]
    pub unsafe fn unsafe_more_features(&self) {}
}

// The current type gains a defaulted const parameter. Callers can still name Arity<T>.
pub struct Arity<T>(T);

impl<T> Arity<T> {
    pub fn value() -> u8 {
        0
    }
}

// A lifetime parameter permits a fixed export name because it does not cause
// separate monomorphizations.
pub struct Exported<'a>(&'a ());

impl<'a> Exported<'a> {
    #[unsafe(export_name = "unrestricted_inherent_impls_old_symbol")]
    pub extern "C" fn symbol() {}
}

// The adapter cannot establish full coverage for the Copy-bound impl in each
// pair. The other release's unrestricted impl is insufficient to report a match.
pub struct Unknown<T>(pub T);

impl<T: Copy> Unknown<T> {
    pub fn baseline_bound(&self) {}
}

impl<T> Unknown<T> {
    pub fn current_bound(&self) {}
}
