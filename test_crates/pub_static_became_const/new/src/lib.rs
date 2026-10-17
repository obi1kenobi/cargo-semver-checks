#![no_std]

// Should be reported.
pub const STATIC_BECOMES_CONST: i32 = 0;
pub const MUT_STATIC_BECOMES_CONST: i32 = 0;

pub mod my_module {
    // Should be reported.
    pub const STATIC_IN_MODULE_BECOMES_CONST: i32 = 0;
}

mod private_module {
    // Should be reported, since it's re-exported at the crate root.
    pub const REEXPORTED_STATIC_BECOMES_CONST: i32 = 0;

    // Should not be reported, since it isn't public API.
    pub const UNREACHABLE_STATIC_BECOMES_CONST: i32 = 0;
}

pub use private_module::REEXPORTED_STATIC_BECOMES_CONST;

// Should not be reported, since it isn't public.
const PRIVATE_STATIC_BECOMES_CONST: i32 = 0;

// Should not be reported, since it wasn't public API in the baseline.
#[doc(hidden)]
pub const DOC_HIDDEN_STATIC_BECOMES_CONST: i32 = 0;

// Should not be reported, since it isn't public API anymore.
#[doc(hidden)]
pub const STATIC_BECOMES_DOC_HIDDEN_CONST: i32 = 0;

// Should not be reported by this lint, since the const isn't public.
// This is covered by `pub_static_missing` instead.
const STATIC_BECOMES_PRIVATE_CONST: i32 = 0;

// Should not be reported, since the item stays a static or was already a const.
pub static STATIC_STAYS_STATIC: i32 = 0;
pub const CONST_STAYS_CONST: i32 = 0;
pub static CONST_BECOMES_STATIC: i32 = 0;
