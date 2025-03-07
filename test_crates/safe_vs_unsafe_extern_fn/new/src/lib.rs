#![no_std]

#![feature(unsafe_extern_blocks)]  // Remove after our lowest tested Rust version is 1.82+.

unsafe extern "C" {
    pub fn originally_legacy_now_unsafe_extern_and_implicit_unsafe();

    pub unsafe fn originally_legacy_now_unsafe_extern_and_explicit_unsafe();

    pub safe fn originally_legacy_now_unsafe_extern_and_safe();
}

unsafe extern "C" {
    pub fn originally_explicit_now_implicit_unsafe();

    pub unsafe fn originally_implicit_now_explicit_unsafe();

    pub fn originally_safe_now_implicit_unsafe();

    pub unsafe fn originally_safe_now_explicit_unsafe();
}

// This is the "legacy" way of specifying `extern`.
extern "C" {
    pub fn originally_implicit_unsafe_now_legacy();

    pub fn originally_explicit_unsafe_now_legacy();

    // This is a breaking change!
    pub fn originally_safe_now_legacy();
}
