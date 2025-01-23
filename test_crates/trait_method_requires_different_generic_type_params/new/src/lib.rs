mod private {
    pub trait Sealed {}
}

// Seal this trait so that we don't also get impl-side breakage reported.
pub trait Example: private::Sealed {
    fn becomes_generic<T>(data: T) -> T;

    fn gains_generic_types<T, U>(data: T) -> U;

    fn loses_generic_types<T>(data: T) -> T;
}
