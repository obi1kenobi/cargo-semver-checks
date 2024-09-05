#[cfg(not(all(feature = "std", feature = "alloc")))]
compile_error!("`std` and `alloc` features are  currently required to build this awesome crate");

pub fn foo_becomes_gated() {}
pub fn bar_becomes_gated() {}

#[cfg(any(
    feature = "unstable",
    feature = "nightly",
    feature = "bench",
    feature = "no_std",
    feature = "__foo",
    feature = "unstable-foo",
    feature = "unstable_foo",
    feature = "_bar"
))]
pub fn unstable_function() {}
