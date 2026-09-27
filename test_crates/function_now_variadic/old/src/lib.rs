#![no_std]
// Rustdoc fixtures also run on Rust versions predating
// stabilization of C-variadic definitions in Rust 1.99.
// TODO: Remove the feature gate and stable_features allowance when Rust 1.99 is the MSRV.
#![feature(c_variadic)]
#![allow(stable_features)]

// These public definitions should be reported.
pub unsafe extern "C" fn no_fixed_definition() {}
pub unsafe extern "C" fn fixed_definition(_: u32) {}
pub unsafe extern "C-unwind" fn unwind_definition(_: u32) {}

// Fixed parameter counts and variadic status are independent signature changes.
// Both lints should report these: reverting either change still breaks function pointers.
pub unsafe extern "C" fn added_fixed_parameter(_first: i32) {}
// Removing a fixed parameter while adding `...` preserves existing two-argument calls,
// but assignments to the previous function pointer type still break.
pub unsafe extern "C" fn removed_fixed_parameter(_first: i32, _second: i32) {}

// Extern declarations can constrain downstream code even when the downstream crate supplies
// the symbol. For example, this baseline API and dependent crate compile together:
//
// Upstream:
// unsafe extern "C" { pub fn provided_by_downstream(value: u32); }
//
// Downstream:
// #[unsafe(export_name = "provided_by_downstream")]
// pub unsafe extern "C" fn implementation(_: u32) {}
// fn witness() {
//     let _: unsafe extern "C" fn(u32) = upstream::provided_by_downstream;
// }
//
// Changing only the upstream declaration's variadic status breaks this pointer assignment.
// All these public declarations should be reported, including the safe one.
unsafe extern "C" {
    pub fn provided_by_downstream(value: u32);
    pub unsafe fn explicit_unsafe_declaration(value: u32);
    pub safe fn safe_declaration(value: u32);
    pub fn no_fixed_declaration();
}

unsafe extern "C-unwind" {
    pub fn unwind_declaration(value: u32);
}

unsafe extern "system" {
    pub fn system_declaration(value: u32);
}

mod implementation {
    pub unsafe extern "C" fn reexported(_: u32) {}

    // These have no public API path and should not be reported.
    pub unsafe extern "C" fn inaccessible(_: u32) {}
    pub unsafe extern "C" fn hidden_reexport(_: u32) {}

    // The old version has no public API path, so this should not be reported.
    pub unsafe extern "C" fn newly_reexported(_: u32) {}
}

// A doc-hidden reexport should not be reported.
#[doc(hidden)]
pub use implementation::hidden_reexport;

// A public reexport from a private module should be reported.
pub use implementation::reexported;

// Private and doc-hidden functions should not be reported.
unsafe extern "C" fn private_function(_: u32) {}
#[doc(hidden)]
pub unsafe extern "C" fn doc_hidden_function(_: u32) {}

// Gaining public visibility or losing doc-hidden status should not be reported.
unsafe extern "C" fn newly_public(_: u32) {}
#[doc(hidden)]
pub unsafe extern "C" fn newly_doc_visible(_: u32) {}

// Losing public API exposure is covered by other lints, not the variadic lints.
pub unsafe extern "C" fn becomes_private(_: u32) {}
pub unsafe extern "C" fn becomes_doc_hidden(_: u32) {}

// Adding or removing the whole function should not be reported by variadic lints.
pub unsafe extern "C" fn removed_function(_: u32) {}

// The opposite variadic-status change should only be reported by the other directional lint.
pub unsafe extern "C" fn opposite_direction(_: u32, _: ...) {}
