#![no_std]
#![allow(internal_features)]
#![feature(associated_type_defaults)]
#![feature(rustc_attrs)]
#![feature(staged_api)]
#![stable(feature = "stability_aware_mode_fixture", since = "1.0.0")]

#[stable(feature = "stability_aware_mode_stable_kept", since = "1.0.0")]
pub fn stable_kept() {}

#[stable(feature = "stability_aware_mode_const_stable", since = "1.0.0")]
pub fn const_stable_to_non_const() -> usize {
    1
}

#[stable(feature = "stability_aware_mode_const_facet", since = "1.0.0")]
pub fn const_unstable_to_non_const() -> usize {
    1
}

#[stable(feature = "stability_aware_mode_doc_hidden", since = "1.0.0")]
#[doc(hidden)]
pub fn stable_now_doc_hidden() {}

#[stable(feature = "stability_aware_mode_default_stability", since = "1.0.0")]
pub trait DefaultStability {
    #[stable(feature = "stability_aware_mode_stable_default_body", since = "1.0.0")]
    fn stable_default_body_removed(&self);

    #[stable(feature = "stability_aware_mode_unstable_default_body", since = "1.0.0")]
    fn unstable_default_body_removed(&self);

    #[stable(feature = "stability_aware_mode_stable_default_const", since = "1.0.0")]
    const STABLE_DEFAULT_CONST_REMOVED: usize;

    #[stable(feature = "stability_aware_mode_unstable_default_const", since = "1.0.0")]
    const UNSTABLE_DEFAULT_CONST_REMOVED: usize;

    #[stable(feature = "stability_aware_mode_stable_default_type", since = "1.0.0")]
    type StableDefaultTypeRemoved;

    #[stable(feature = "stability_aware_mode_unstable_default_type", since = "1.0.0")]
    type UnstableDefaultTypeRemoved;

    #[stable(
        feature = "stability_aware_mode_stable_default_body_to_unstable",
        since = "1.0.0"
    )]
    #[rustc_default_body_unstable(
        feature = "stability_aware_mode_stable_default_body_now_unstable",
        issue = "none"
    )]
    fn stable_default_body_to_unstable(&self) {}
}

#[unstable(
    feature = "stability_aware_mode_stable_item_now_unstable",
    issue = "none"
)]
pub fn stable_item_to_unstable() {}

#[stable(
    feature = "stability_aware_mode_const_stable_to_unstable",
    since = "1.0.0"
)]
#[rustc_const_unstable(
    feature = "stability_aware_mode_const_stable_now_unstable",
    issue = "none"
)]
pub const fn const_stable_to_unstable() -> usize {
    1
}
