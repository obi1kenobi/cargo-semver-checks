pub trait NonSealed {
    /// Removing an associated type is breaking when the trait is not hidden nor sealed,
    /// even if the associated type was hidden.
    #[doc(hidden)]
    type T;

    /// Changing the bounds on an associated type is breaking,
    /// even if the type was hidden.
    #[doc(hidden)]
    type Bounded: Clone;

    /// Removing an associated const without a default value in a non-sealed trait is breaking,
    /// even if the const was hidden.
    ///
    /// This is because the trait could have been implemented by external users,
    /// and such implementations were *required* to specify a value for this const.
    /// It is that const value statement that will be broken.
    #[doc(hidden)]
    const WITHOUT_DEFAULT: i64;

    /// Removing an associated const with a default value is *not* breaking,
    /// even if the const was hidden.
    ///
    /// This is because the trait was possible to implement without redefining this constant.
    /// Since the constant was hidden from the public API, any use of it was non-conformant
    /// and constituted use of a non-public API.
    #[doc(hidden)]
    const WITH_DEFAULT: i64 = 5;

    /// Removing the default value for a hidden associated const of a non-sealed trait is breaking:
    /// any implementations of this trait are now required to specify a value for the const.
    #[doc(hidden)]
    const DEFAULT_REMOVED: i64 = 5;

    /// Removing an associated function without a default impl in a non-sealed trait is breaking,
    /// under the same reasoning as for associated constants without a default.
    #[doc(hidden)]
    fn implement_me();

    /// Changing a function signature without a default impl in a non-sealed trait is breaking,
    /// under the same reasoning as for associated constants without a default.
    #[doc(hidden)]
    fn changed_signature(x: i64, y: i64);

    /// Removing a default impl for a function in a non-sealed trait is breaking,
    /// even if the function was hidden: external implementors must now provide an implementation.
    #[doc(hidden)]
    fn default_impl_removed(x: i64, y: i64) -> i64 {
        x + y
    }
}

mod private {
    pub trait Sealed {}
}

pub trait SealedViaSupertrait : private::Sealed {
    /// Removing a hidden associated type is not breaking when the trait is sealed.
    #[doc(hidden)]
    type T;

    /// Changing the bounds on a hidden associated type of a sealed trait is not breaking per se.
    #[doc(hidden)]
    type Bounded: Clone;

    /// Removing a hidden associated const value in a sealed trait is not breaking,
    /// regardless of the presence of a default value.
    #[doc(hidden)]
    const WITHOUT_DEFAULT: i64;

    /// Removing a hidden associated const value in a sealed trait is not breaking,
    /// regardless of the presence of a default value.
    #[doc(hidden)]
    const WITH_DEFAULT: i64 = 5;

    /// Removing the default value for a hidden associated const of a sealed trait is not breaking,
    /// regardless of the presence of a default value, since all implementors of the trait
    /// are required to be in the same crate.
    #[doc(hidden)]
    const DEFAULT_REMOVED: i64 = 5;

    /// Removing a hidden associated function in a sealed trait is not breaking,
    /// regardless of the presence of a default implementation.
    /// All implementors of the trait are required to be in the same crate.
    #[doc(hidden)]
    fn implement_me();

    /// Changing a hidden function's signature in a sealed trait is not breaking,
    /// since all implementors of this trait are required to be in the same crate.
    #[doc(hidden)]
    fn changed_signature(x: i64, y: i64);

    /// Removing a default impl for a hidden function in a sealed trait is not breaking,
    /// since all implementors of this trait are required to be in the same crate.
    #[doc(hidden)]
    fn default_impl_removed(x: i64, y: i64) -> i64 {
        x + y
    }
}
