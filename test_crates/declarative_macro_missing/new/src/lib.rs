#![no_std]

macro_rules! will_no_longer_be_exported {
    () => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! became_doc_hidden {
    () => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! always_doc_hidden {
    () => {};
}
