mod MyNonPublicMod {
    // despite adding #[doc(hidden)], this struct is in a
    // private mod, so it isn't part of the crate's public
    // api
    #[doc(hidden)]
    pub struct MyStruct;
}

pub mod MyPublicMod {
    // added #[doc(hidden)], however this struct is in a
    // public mod, so it is part of the crate's public api
    #[doc(hidden)]
    pub struct MyStruct;
}

#[doc(hidden)]
pub struct Example;
