#[cfg(feature = "foo")]
pub fn feature_dependant_function() {}

#[cfg(any(
    feature = "unstable",
    feature = "nightly",
    feature = "bench",
    feature = "no_std",
    feature = "__foo"
))]
pub fn unstable_function() {}
