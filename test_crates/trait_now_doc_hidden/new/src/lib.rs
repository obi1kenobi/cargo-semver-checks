mod MyNonPublicMod {
    // despite adding #[doc(hidden)], this trait is in a
    // private mod, so it isn't part of the crate's public
    // api
    #[doc(hidden)]
    pub trait MyTrait {}
}

pub mod MyPublicMod {
    // added #[doc(hidden)], however this trait is in a
    // public mod, so it previously was part of the crate's public api
    #[doc(hidden)]
    pub trait MyTrait {}
}

#[doc(hidden)]
pub mod MyTopLevelDocHiddenMod {
    #[doc(hidden)] // this shouldn't flag, as it's a top level mod
                   // was never part of the public api of the crate
    pub trait MyTraitThatIsNowDocHidden {}
}

mod MyNestedNonPublicMod {
    pub mod PublicInnerMod {
        // despite adding #[doc(hidden)], this trait is in a
        // private outer mod, so it isn't part of the crate's public
        // api
        #[doc(hidden)]
        pub trait MyTrait {}
    }
}

pub mod MyNestedPublicMod {
    pub mod PublicInnerMod {
        // added #[doc(hidden)], however this trait is in a
        // public mod, so it previously was part of the crate's public api
        #[doc(hidden)]
        pub trait MyTrait {}
    }
}

#[doc(alias = "hidden")] // shouldn't flag, this is just aliased as hidden,
                         // but it should be #[doc(hidden)]
pub trait AliasedAsDocHidden {}

#[doc(hidden)] // should flag, this is the simplest case of adding #[doc(hidden)] to a pub trait.
pub trait Example {}

pub trait PublicTraitHiddenVariant {
    // shouldn't flag `trait_now_doc_hidden` rule
    // as this is a trait-fn that's hidden,
    // not the entire trait
    #[doc(hidden)]
    fn my_trait_fn(&self);
}

#[doc(hidden)]
trait PublicTraitThatGoesPrivate {}

#[doc = "hidden"] // shouldn't flag, this is just documented with the string "hidden",
                  // it's not actually #[doc(hidden)]
pub trait PublicTraitDocumentedWithStringHidden {}
