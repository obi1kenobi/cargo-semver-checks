mod MyNonPublicMod {
    // despite adding #[doc(hidden)], this struct is in a
    // private mod, so it isn't part of the crate's public
    // api
    #[doc(hidden)]
    pub struct MyStruct;
}

pub mod MyPublicMod {
    // added #[doc(hidden)], however this struct is in a
    // public mod, so it previously was part of the crate's public api
    #[doc(hidden)]
    pub struct MyStruct;
}

#[doc(hidden)]
pub mod MyTopLevelDocHiddenMod {
    #[doc(hidden)] // this shouldn't flag, as it's a top level mod
                   // was never part of the public api of the crate
    pub struct MyStructThatIsNowDocHidden;
}

mod MyNestedNonPublicMod {
    pub mod PublicInnerMod {
        // despite adding #[doc(hidden)], this struct is in a
        // private outer mod, so it isn't part of the crate's public
        // api
        #[doc(hidden)]
        pub struct MyStruct;
    }
}

pub mod MyNestedPublicMod {
    pub mod PublicInnerMod {
        // added #[doc(hidden)], however this struct is in a
        // public mod, so it previously was part of the crate's public api
        #[doc(hidden)]
        pub struct MyStruct;
    }
}

#[doc(alias = "hidden")] // shouldn't flag, this is just aliased as hidden,
                         // but it should be #[doc(hidden)]
pub struct AliasedAsDocHidden;

#[doc(hidden)] // should flag, this is the simplest case of adding #[doc(hidden)] to a pub struct.
pub struct Example;

pub struct PublicStructHiddenField {
    // shouldn't flag `struct_now_doc_hidden` rule
    // as this is a field that's hidden,
    // not the entire struct
    #[doc(hidden)]
    pub my_field: i8,
}

#[doc(hidden)]
struct PublicStructThatGoesPrivate;

#[doc = "hidden"] // shouldn't flag, this is just documented with the string "hidden",
                  // it's not actually #[doc(hidden)]
pub struct PublicStructDocumentedWithStringHidden;
