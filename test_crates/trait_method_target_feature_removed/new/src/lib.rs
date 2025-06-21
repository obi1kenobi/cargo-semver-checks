#![no_std]

mod private {
    pub trait Sealed {}

    pub trait ReExportNoLongerSealed {}

    #[doc(hidden)]
    pub trait FromHiddenToSealed {}
}

pub use private::ReExportNoLongerSealed;

#[doc(hidden)]
pub trait Hidden {}

pub trait NoLongerHidden {}

// This should trigger `trait_method_target_feature_removed`.
pub trait TraitA {
    unsafe fn feature_removed(&self) {}
}

// Nothing changed here, no lint.
pub trait TraitB {
    #[target_feature(enable = "avx2")]
    unsafe fn still_featured(&self) {}
}

// `avx` was removed, this should trigger `trait_method_target_feature_removed`.
pub trait TraitE {
    #[target_feature(enable = "bmi1")]
    unsafe fn partial_feature_removed(&self) {}
}

// `avx` was removed but it's still implied by `avx2`, no lint.
pub trait TraitF {
    #[target_feature(enable = "avx2")]
    unsafe fn consolidated(&self) {}
}

// `sse2` is still globally enabled, no lint.
pub trait TraitC {
    unsafe fn remove_globally_enabled_feature(&self) {}
}

// The trait is sealed so no downstream impls to worry about, no lint.
pub trait TraitD: private::Sealed {
    unsafe fn sealed_trait_feature_removed(&self) {}
}

// The trait is public API sealed,
// `pub_api_sealed_trait_method_target_feature_removed` should lint here.
pub trait PublicApiSealed: Hidden {
    unsafe fn feature_removed(&self) {}
}

// The trait is private, nothing to report here.
trait PrivateTrait {
    unsafe fn private_feature_removed(&self) {}
}

// The trait used to be public API sealed, but no longer is.
// There might still be downstream impls (even though they were going through non-pub API)
// so `pub_api_sealed_trait_method_target_feature_removed` should lint here.
pub trait NoLongerPublicApiSealed: NoLongerHidden {
    unsafe fn feature_removed(&self) {}
}

// This trait used to be sealed, so no downstream impls exist.
// It doesn't matter that it's unsealed now, there shouldn't be any lints here.
pub trait NoLongerSealed: private::ReExportNoLongerSealed {
    unsafe fn sealed_trait_feature_removed(&self) {}
}

// The trait newly became sealed. All downstream impls are broken anyway,
// so nothing to report here.
pub trait FeatureRemovedAndSealed: private::Sealed {
    unsafe fn feature_removed(&self) {}
}

// The trait newly became public API sealed. But existing impls still exist,
// and may be broken by the removed feature. This should trigger
// the `trait_method_target_feature_removed` lint.
pub trait FeatureRemovedAndPubApiSealed: Hidden {
    unsafe fn feature_removed(&self) {}
}

// The trait newly became sealed. All downstream impls are broken anyway,
// so nothing to report here.
pub trait FeatureRemovedFromPubApiSealedToUnconditional: private::FromHiddenToSealed {
    unsafe fn feature_removed(&self) {}
}
