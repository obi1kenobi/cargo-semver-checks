mod MyNonPublicMod {
    pub fn my_function() {}
}

pub mod MyPublicMod {
    pub fn my_function() {}
}

#[doc(hidden)]
pub mod MyTopLevelDocHiddenMod {
    pub fn my_function() {}
}

mod MyNestedNonPublicMod {
    pub mod PublicInnerMod {
        pub fn my_function() {}
    }
}

pub mod MyNestedPublicMod {
    pub mod PublicInnerMod {
        pub fn my_function() {}
    }
}

pub fn aliased_as_doc_hidden() {}

pub fn my_function() {}

pub fn my_function_with_types(a: i32, b: i32) -> i32 {}

fn public_function_that_goes_private() {}

pub fn public_function_documented_with_string_hidden() {}

const MY_FN: fn() = || {};
