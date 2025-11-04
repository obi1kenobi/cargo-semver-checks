#![no_std]

mod private {
    pub trait Sealed {}

    pub trait ReExportNoLongerSealed {}

    #[doc(hidden)]
    pub trait FromHiddenToSealed {}
}

pub use private::FromHiddenToSealed;

#[doc(hidden)]
pub trait Hidden {}

#[doc(hidden)]
pub trait NoLongerHidden {}

pub trait TraitA {
    #[target_feature(enable = "avx2")]
    unsafe fn feature_removed(&self) {}
}

pub trait TraitB {
    #[target_feature(enable = "avx2")]
    unsafe fn still_featured(&self) {}
}

pub trait TraitE {
    #[target_feature(enable = "avx", enable = "bmi1")]
    unsafe fn partial_feature_removed(&self) {}
}

pub trait TraitF {
    #[target_feature(enable = "avx", enable = "avx2")]
    unsafe fn consolidated(&self) {}
}

pub trait TraitC {
    #[target_feature(enable = "sse2")]
    unsafe fn remove_globally_enabled_feature(&self) {}
}

pub trait TraitD: private::Sealed {
    #[target_feature(enable = "avx2")]
    unsafe fn sealed_trait_feature_removed(&self) {}
}

pub trait PublicApiSealed: Hidden {
    #[target_feature(enable = "bmi1")]
    unsafe fn feature_removed(&self) {}
}

trait PrivateTrait {
    #[target_feature(enable = "avx2")]
    unsafe fn private_feature_removed(&self) {}
}

pub trait NoLongerPublicApiSealed: NoLongerHidden {
    #[target_feature(enable = "bmi1")]
    unsafe fn feature_removed(&self) {}
}

pub trait NoLongerSealed: private::ReExportNoLongerSealed {
    #[target_feature(enable = "bmi1")]
    unsafe fn sealed_trait_feature_removed(&self) {}
}

pub trait FeatureRemovedAndSealed {
    #[target_feature(enable = "bmi1")]
    unsafe fn feature_removed(&self) {}
}

pub trait FeatureRemovedAndPubApiSealed {
    #[target_feature(enable = "bmi1")]
    unsafe fn feature_removed(&self) {}
}

pub trait FeatureRemovedFromPubApiSealedToUnconditional: private::FromHiddenToSealed {
    #[target_feature(enable = "bmi1")]
    unsafe fn feature_removed(&self) {}
}
