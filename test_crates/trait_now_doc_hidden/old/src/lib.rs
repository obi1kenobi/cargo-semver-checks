mod MyNonPublicMod {
    pub trait MyTrait {}
}

pub mod MyPublicMod {
    pub trait MyTrait {}
}

#[doc(hidden)]
pub mod MyTopLevelDocHiddenMod {
    pub trait MyTraitThatIsNowDocHidden {}
}

mod MyNestedNonPublicMod {
    pub mod PublicInnerMod {
        pub trait MyTrait {}
    }
}

pub mod MyNestedPublicMod {
    pub mod PublicInnerMod {
        pub trait MyTrait {}
    }
}

pub trait AliasedAsDocHidden {}

pub trait Example {}

pub trait PublicTraitHiddenVariant {
    fn my_trait_fn(&self);
}

trait PublicTraitThatGoesPrivate {}

pub trait PublicTraitDocumentedWithStringHidden {}
