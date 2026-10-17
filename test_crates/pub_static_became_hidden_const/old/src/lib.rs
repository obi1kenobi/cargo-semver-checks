#![no_std]

// Should be reported.
pub static STATIC_BECOMES_HIDDEN_CONST: i32 = 0;
pub static mut MUT_STATIC_BECOMES_HIDDEN_CONST: i32 = 0;

pub mod my_module {
    // Should be reported.
    pub static STATIC_IN_MODULE_BECOMES_HIDDEN_CONST: i32 = 0;
}

// Should not be reported, since it wasn't public API in the baseline.
#[doc(hidden)]
pub static HIDDEN_STATIC_BECOMES_HIDDEN_CONST: i32 = 0;

// Should not be reported, since the const stays public API.
// This is covered by `pub_static_became_const` instead.
pub static STATIC_BECOMES_CONST: i32 = 0;

// Should not be reported, since the const isn't public.
// This is covered by `pub_static_missing` instead.
pub static STATIC_BECOMES_PRIVATE_CONST: i32 = 0;

// Should not be reported, since the item stays a static or was already a const.
pub static STATIC_BECOMES_HIDDEN_STATIC: i32 = 0;
pub const CONST_BECOMES_HIDDEN_CONST: i32 = 0;
