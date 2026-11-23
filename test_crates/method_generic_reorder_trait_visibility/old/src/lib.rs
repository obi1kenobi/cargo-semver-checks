#![no_std]

pub struct Owner;

trait Private {
    fn only_private<const N: usize, T>() {}
    fn reordered<T, const N: usize>() {}
}
impl Private for Owner {}

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

impl HiddenPath for Owner {}
impl private::Anonymous for Owner {}
impl Named for Owner {}

impl Owner {
    pub fn only_private<T, const N: usize>() {}
    pub fn only_hidden<T, const N: usize>() {}
    pub fn only_anonymous<T, const N: usize>() {}
    pub fn reordered<T, const N: usize>() {}
    pub fn preserved<T, const N: usize>() {}
}
