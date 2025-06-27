#![no_std]

mod MyNonPublicMod {
    // despite adding #[doc(hidden)], this function is in a
    // private mod, so it isn't part of the crate's public
    // api
    #[doc(hidden)]
    pub fn my_function() {}
}

pub mod MyPublicMod {
    // added #[doc(hidden)], however this function is in a
    // public mod, so it previously was part of the crate's public api
    #[doc(hidden)]
    pub fn my_function() {}
}

#[doc(hidden)]
pub mod MyTopLevelDocHiddenMod {
    #[doc(hidden)] // this shouldn't flag, as it's a top level mod
                   // was never part of the public api of the crate
    pub fn my_function() {}
}

mod MyNestedNonPublicMod {
    pub mod PublicInnerMod {
        // despite adding #[doc(hidden)], this function is in a
        // private outer mod, so it isn't part of the crate's public
        // api
        #[doc(hidden)]
        pub fn my_function() {}
    }
}

pub mod MyNestedPublicMod {
    pub mod PublicInnerMod {
        // added #[doc(hidden)], however this function is in a
        // public mod, so it previously was part of the crate's public api
        #[doc(hidden)]
        pub fn my_function() {}
    }
}

#[doc(alias = "hidden")] // shouldn't flag, this is just aliased as hidden,
                         // but it should be #[doc(hidden)]
pub fn aliased_as_doc_hidden() {}

#[doc(hidden)] // should flag, this is the simplest case of adding #[doc(hidden)] to a pub function.
pub fn my_function() {}

#[doc(hidden)] // should flag, this is the case of adding #[doc(hidden)] to a pub function with arguments and a return type.
pub fn my_function_with_types(a: i32, b: i32) -> i32 {
    a + b
}

#[doc(hidden)]
fn public_function_that_goes_private() {}

#[doc = "hidden"] // shouldn't flag, this is just documented with the string "hidden",
                  // it's not actually #[doc(hidden)]
pub fn public_function_documented_with_string_hidden() {}

#[doc(hidden)] // shouldn't flag under the `function_now_doc_hidden` lint,
               // this is a constant with a function value, not a real function
pub const MY_FN: fn() = || {};

fn my_private_fn() {
    #[doc(hidden)] // shouldn't flag because functions aren't hoisted out of nested scopes
    pub fn my_public_inner_fn_inside_my_private_fn() {}
}

pub fn my_public_fn() {
    #[doc(hidden)] // shouldn't flag because functions aren't hoisted out of nested scopes
    pub fn my_public_inner_fn_inside_my_public_fn() {}
}
