use std::{cell::RefCell, rc::Rc};

// Reversed old/new tests from auto_trait_impl_removed

pub struct SyncStruct {
    bar: usize,
}

pub struct SendStruct {
    // RefCell<T> is Send if T is Send, but it is never Sync.
    bar: RefCell<usize>,
}

pub struct UnwindSafeStruct<'a> {
    bar: &'a i64,
}

pub struct RefUnwindSafeStruct {
    bar: Rc<i64>,
}

pub struct UnpinStruct {
    bar: i64,
}

// #[doc(hidden)] should not be flagged
#[doc(hidden)]
pub struct HiddenSyncStruct {
    bar: usize,
}

// Non-public types should not be flagged
pub(crate) struct PrivateSyncStruct {
    bar: usize,
}
