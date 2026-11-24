#![no_std]

pub struct Owner;

trait Private {
    fn only_private<const N: usize, T>() {}
    fn reordered<T, const N: usize>() {}
}

mod private {
    pub trait HiddenPath {
        fn only_hidden<const N: usize, T>() {}
    }

    pub trait Anonymous {
        fn only_anonymous<const N: usize, T>() {}
        fn preserved<T, const N: usize>() {}
        fn reordered<const OTHER: usize, U>() {}
    }

    pub trait Named {
        fn reordered<const N: usize, T>() {}
        fn preserved<const N: usize, T>() {}
    }
}

#[doc(hidden)]
pub use private::HiddenPath;
pub use private::Named;
pub use private::Named as NamedAlias;

pub mod prelude {
    pub use crate::private::Anonymous as _;
}

// Neither `Private` nor `HiddenPath` has a public API path, so their methods cannot
// establish a reorder. The private trait also cannot preserve the old order of `Owner::reordered`.
impl Private for Owner {}
impl HiddenPath for Owner {}

// A prelude glob still allows `Owner::preserved::<u8, 1>()`. It also exposes
// the new argument order of `Owner::only_anonymous`, but the adapter omits
// the anonymous re-export from importable paths, so the reorder lints miss that change.
impl private::Anonymous for Owner {}

// Report `Owner::reordered` once per changed position, citing only `Named::reordered`.
// Both public names refer to the same declaration.
impl Named for Owner {}
