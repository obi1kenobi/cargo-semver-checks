#![no_std]

#[doc(hidden)]
pub trait Example {
    /// Changing the bounds on an associated type is not breaking for hidden traits.
    type Bounded: Send + Sync;

    /// Changing the type of an associated const is not breaking for hidden traits.
    const CHANGED: &'static str;

    /// Removing the default value for an associated const is not breaking for hidden traits.
    const WITH_DEFAULT: i64;

    /// Changing the default value of an associated const is not breaking for hidden traits.
    const THE_ANSWER: i64 = 53;

    /// Changing a function signature is not breaking for hidden traits.
    fn changed_signature(x: i64, y: i64, z: i64) -> i64;

    /// Removing a default impl for a function is not breaking for hidden traits.
    fn default_impl_removed(x: i64, y: i64) -> i64;
}
