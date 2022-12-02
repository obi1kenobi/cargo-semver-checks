use std::{
    cell::RefCell,
    marker::PhantomPinned,
    panic::{AssertUnwindSafe, UnwindSafe},
    rc::Rc,
    sync::Arc,
};

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
