#![no_std]
// Rustdoc fixtures also run on Rust versions predating
// stabilization of C-variadic definitions in Rust 1.99.
// TODO: Remove the feature gate and stable_features allowance when Rust 1.99 is the MSRV.
#![feature(c_variadic)]
#![allow(stable_features)]

// Exported symbols are part of the ABI regardless of their Rust visibility,
// so most cases live in a private module to keep them out of the Rust-path lints.
mod abi_only {
    // These exported functions stop being variadic and should be reported.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn no_longer_variadic(_: i32, _: ...) {}

    #[unsafe(export_name = "explicit_export_no_longer_variadic")]
    pub unsafe extern "C" fn explicitly_exported(_: i32, _: ...) {}

    #[unsafe(no_mangle)]
    pub unsafe extern "C-unwind" fn unwind_no_longer_variadic(_: i32, _: ...) {}

    pub struct Exporter;

    impl Exporter {
        #[unsafe(export_name = "method_no_longer_variadic")]
        pub unsafe extern "C" fn method(_: i32, _: ...) {}
    }

    // Not reported: the exported function stays non-variadic.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn still_fixed(_: i32) {}

    // Not reported: the exported function stays variadic.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn still_variadic(_: i32, _: ...) {}

    // Not reported: the function is not exported, so it has no symbol-level ABI.
    pub unsafe extern "C" fn not_exported(_: i32, _: ...) {}

    // Not reported: the export is removed, which is a different breaking change.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn export_removed(_: i32, _: ...) {}

    // Not reported: the symbol is only exported in the new version.
    pub unsafe extern "C" fn export_added(_: i32, _: ...) {}

    // Not reported: the export name moves between two functions that are both non-variadic.
    #[unsafe(export_name = "moved_between_fixed")]
    pub unsafe extern "C" fn first_fixed(_: i32) {}

    pub unsafe extern "C" fn second_fixed(_: i32) {}

    // Not reported here: the opposite change belongs to `exported_function_now_variadic`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn now_variadic(_: i32) {}
}

// The export name moves from a variadic trait impl method back to a non-variadic one.
// No Rust-path lint sees this, since the trait itself is unchanged; it should be reported.
pub trait Hooks: Sized {
    unsafe extern "C" fn fixed(value: i32);
    unsafe extern "C" fn variadic(value: i32, _: ...);
}

pub struct Provider;

impl Hooks for Provider {
    unsafe extern "C" fn fixed(_value: i32) {}

    #[unsafe(export_name = "hook")]
    unsafe extern "C" fn variadic(_value: i32, _: ...) {}
}
