pub struct Plain {
    #[doc(hidden)]
    pub field: i64,
}

pub struct Tuple(#[doc(hidden)] pub i64);

/// Both the struct and its field here will become `#[doc(hidden)]`.
///
/// This is a rare case where we want to report a lint for both the struct and the field.
/// Doc-hiddenness on the struct means we can't legally *name* it (i.e. import & use it).
/// But if an existing public API returns this struct, its pub fields can still be public API
/// without naming the struct's type.
#[doc(hidden)]
pub struct BothBecomeHidden {
    #[doc(hidden)]
    pub field: i64,
}
