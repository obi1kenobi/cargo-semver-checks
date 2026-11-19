#![no_std]

pub struct VisibleIterator;
pub struct HiddenIterator;

// Downstream code can use either type in a `for` loop, including the hidden impl.
impl Iterator for VisibleIterator {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[doc(hidden)]
impl Iterator for HiddenIterator {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
