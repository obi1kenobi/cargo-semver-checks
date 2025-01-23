mod private {
    pub trait Sealed {}
}

// Seal this trait so that we don't also get impl-side breakage reported.
pub trait Example: private::Sealed {
    fn becomes_generic(data: i64) -> i64;

    fn gains_generic_types<T>(data: T) -> T;

    fn loses_generic_types<T, U: From<T>>(data: T) -> U;

    // Not a major breaking change!
    // https://predr.ag/blog/some-rust-breaking-changes-do-not-require-major-version/
    fn becomes_impl_trait(data: String) {}
}
