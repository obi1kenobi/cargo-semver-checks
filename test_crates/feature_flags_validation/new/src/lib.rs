#[cfg(feature = "foo")]
pub fn foo_becomes_gated() {}

#[cfg(feature = "bar")]
pub fn bar_becomes_gated() {}

#[cfg(any(feature = "unstable", feature = "nightly",))]
pub fn unstable_function() {}
