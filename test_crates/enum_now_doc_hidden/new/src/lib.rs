mod MyNonPublicMod {
    // despite adding #[doc(hidden)], this enum is in a
    // private mod, so it isn't part of the crate's public
    // api
    #[doc(hidden)]
    pub enum MyEnum {
        A,
    }
}

pub mod MyPublicMod {
    // added #[doc(hidden)], however this enum is in a
    // public mod, so it previously was part of the crate's public api
    #[doc(hidden)]
    pub enum MyEnum {
        A,
    }
}

mod MyNestedNonPublicMod {
    pub mod PublicInnerStruct {
        // despite adding #[doc(hidden)], this enum is in a
        // private outer mod, so it isn't part of the crate's public
        // api
        #[doc(hidden)]
        pub enum MyEnum {
            A,
        }
    }
}

pub mod MyNestedPublicMod {
    pub mod PublicInnerStruct {
        // added #[doc(hidden)], however this enum is in a
        // public mod, so it is part of the crate's public api
        #[doc(hidden)]
        pub enum MyEnum {
            A,
        }
    }
}

#[doc(alias = "hidden")] // shouldn't flag, this is just aliased as hidden,
                         // but it should be #[doc(hidden)]
pub enum AliasedAsDocHidden {
    A,
}

#[doc(hidden)] // should flag, this is the simplest case of adding #[doc(hidden)] to a pub enum.
pub enum Example {
    A,
}

pub enum PublicEnumHiddenVariant {
    // shouldn't flag `enum_now_doc_hidden` rule
    // as this is a field that's hidden,
    // not the entire struct
    #[doc(hidden)]
    A,
    B,
}

pub enum PublicEnumHiddenStructFieldOnVariant {
    // shouldn't flag `enum_now_doc_hidden` rule
    // as this is a field that's hidden on a struct variant,
    // not the entire enum
    A {
        #[doc(hidden)]
        a: u8,
    },
    B,
}

#[doc(hidden)]
enum PublicEnumThatGoesPrivate {
    A,
}

#[doc = "hidden"] // shouldn't flag, this is just documented with the string "hidden",
                  // it's not actually #[doc(hidden)]
pub enum PublicEnumDocumentedWithStringHidden {
    A,
}
