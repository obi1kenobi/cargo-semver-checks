#![no_std]

mod private {
    pub trait Sealed {}
}

// Seal this trait so that we don't also get impl-side breakage reported.
pub trait Example: private::Sealed {
    fn becomes_generic<T>(data: T) -> T;

    fn gains_generic_types<T, U>(data: T) -> U;

    fn loses_generic_types<T>(data: T) -> T;

    // Not a major breaking change!
    // https://predr.ag/blog/some-rust-breaking-changes-do-not-require-major-version/
    fn becomes_impl_trait(data: impl Into<i64>) {}
}
