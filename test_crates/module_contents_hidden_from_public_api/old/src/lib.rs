#![no_std]

pub mod removals {
    #[doc(hidden)]
    pub fn hidden() {}

    #[doc(hidden)]
    pub static VALUE: &'static str = "hidden";

    #[doc(hidden)]
    pub const UNCHANGING: i64 = 42;
}

#[doc(hidden)]
pub mod module_removal {
    pub fn parent_hidden() {}

    pub static OTHER_VALUE: &'static str = "hidden";

    pub const FIXED: i64 = 42;
}

#[doc(hidden)]
pub fn changed_fn_signature(x: i64, y: i64) -> i64 {
    x + y
}
