#[macro_export]
macro_rules! will_be_removed {
    () => {};
}

#[macro_export]
macro_rules! will_no_longer_be_exported {
    () => {};
}

macro_rules! textual_scope_macro_removed {
    () => {};
}

#[macro_export]
macro_rules! became_doc_hidden {
    () => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! always_doc_hidden {
    () => {};
}
