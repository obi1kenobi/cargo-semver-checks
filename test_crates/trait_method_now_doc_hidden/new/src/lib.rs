// Tests adapted from the trait_now_doc_hidden lint and tests

// since this is not a public trait, it isn't semver-breaking if we add
// #[doc(hidden)] to one of the methods
trait NonPublicTrait {
    #[doc(hidden)]
    fn method(&self);
}

// this is a public trait, so it is semver-breaking if we add doc hidden to
// any method
pub trait PublicTrait {
    #[doc(hidden)]
    fn assoc_function();
    #[doc(hidden)]
    fn method(&self);
}

mod NonPublicMod {
    // although this is a public trait, it is part of a non-public module,
    // so it is not semver-breaking to add doc hidden on a method
    pub trait PublicInnerTrait {
        #[doc(hidden)]
        fn method(&self);
    }
}

#[doc(hidden)]
pub mod HiddenMod {
    // although this is in a public module and a public trait, the module
    // has always been marked doc hidden, so it is not a semver breaking
    // change to add doc hidden to an already hidden method
    // (would it do anything?)
    pub trait PublicInnerTrait {
        #[doc(hidden)]
        fn method(&self);
    }
}

// since this is in a hidden trait, hiding a method is not semver breaking
#[doc(hidden)]
pub trait HiddenTrait {
    #[doc(hidden)]
    fn method(&self);
}
