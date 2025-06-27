#![no_std]

pub trait PublicAPISealed {
    /// Changing the bounds on a hidden type of a public API sealed trait is non breaking
    #[doc(hidden)]
    type Bounded: Send + Sync;

    /// Removing the default value for a hidden associated const of a public API sealed trait is not breaking
    #[doc(hidden)]
    const DEFAULT_REMOVED: i64;

    /// Changing a function signature without a default impl in a public API sealed trait is not breaking
    #[doc(hidden)]
    fn changed_signature(x: i64, y: i64, z: i64) -> i64;

    /// Removing a default impl for a function in a public API sealed trait is not breaking
    #[doc(hidden)]
    fn default_impl_removed(x: i64, y: i64) -> i64;
}

mod private {
    pub trait Sealed {}
}

pub trait SealedViaSupertrait : private::Sealed {
    /// Changing the bounds on a hidden associated type of a sealed trait is not breaking per se.
    #[doc(hidden)]
    type Bounded: Send + Sync;

    /// Removing the default value for a hidden associated const of a sealed trait is not breaking,
    /// regardless of the presence of a default value, since all implementors of the trait
    /// are required to be in the same crate.
    #[doc(hidden)]
    const DEFAULT_REMOVED: i64;

    /// Changing a hidden function's signature in a sealed trait is not breaking,
    /// since all implementors of this trait are required to be in the same crate.
    #[doc(hidden)]
    fn changed_signature(x: i64, y: i64, z: i64) -> i64;

    /// Removing a default impl for a hidden function in a sealed trait is not breaking,
    /// since all implementors of this trait are required to be in the same crate.
    #[doc(hidden)]
    fn default_impl_removed(x: i64, y: i64) -> i64;
}
