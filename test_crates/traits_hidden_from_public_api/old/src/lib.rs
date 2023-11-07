#[doc(hidden)]
pub trait Example {
    /// Removing an associated type is not breaking for hidden traits.
    type T;

    /// Changing the bounds on an associated type is not breaking for hidden traits.
    type Bounded: Clone;

    /// Removing an associated const is not breaking for hidden traits.
    const N: i64;

    /// Changing the type of an associated const is not breaking for hidden traits.
    const CHANGED: usize;

    /// Removing the default value for an associated const is not breaking for hidden traits.
    const WITH_DEFAULT: i64 = 5;

    /// Changing the default value of an associated const is not breaking for hidden traits.
    const THE_ANSWER: i64 = 42;

    /// Removing an associated function is not breaking for hidden traits.
    fn implement_me();

    /// Changing a function signature is not breaking for hidden traits.
    fn changed_signature(x: i64, y: i64);

    /// Removing a default impl for a function is not breaking for hidden traits.
    fn default_impl_removed(x: i64, y: i64) -> i64 {
        x + y
    }
}
