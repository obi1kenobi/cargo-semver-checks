use std::{
    cell::RefCell,
    marker::PhantomPinned,
    panic::{AssertUnwindSafe, UnwindSafe},
    rc::Rc,
    sync::Arc,
};

pub struct SyncStruct {
    // RefCell<T> is Send if T is Send, but it is never Sync.
    // We need AssertUnwindSafe in order to keep this struct RefUnwindSafe.
    // We have a RefUnwindSafe-specific test lower down in this file.
    bar: AssertUnwindSafe<RefCell<usize>>,
}

pub struct SendStruct {
    // This is just a silly type that is neither Send nor Sync.
    // It's not actually useful for anything.
    bar: RefCell<Rc<usize>>,
}

pub struct UnwindSafeStruct<'a> {
    // &mut T is not UnwindSafe unless T is AssertUnwindSafe.
    bar: &'a mut i64,
}

pub struct RefUnwindSafeStruct {
    bar: Rc<RefCell<i64>>,
}

// The RefUnwindSafeStruct struct checks for RefUnwindSafe being removed.
// However, the way it's constructed above means that it loses both
// the UnwindSafe and the RefUnwindSafe traits.
//
// Manually put the UnwindSafe trait back in, so that our test case tests
// only for RefUnwindSafe being removed. UnwindSafe was tested earlier
// in this file.
impl UnwindSafe for RefUnwindSafeStruct {}

pub struct UnpinStruct {
    bar: i64,
    _marker: PhantomPinned,
}
