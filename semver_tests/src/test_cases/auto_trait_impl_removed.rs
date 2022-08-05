use std::{
    cell::RefCell,
    marker::PhantomPinned,
    panic::{AssertUnwindSafe, UnwindSafe},
    rc::Rc,
    sync::Arc,
};

#[cfg(not(feature = "auto_trait_impl_removed"))]
pub struct SyncStruct {
    bar: usize,
}

#[cfg(feature = "auto_trait_impl_removed")]
pub struct SyncStruct {
    // RefCell<T> is Send if T is Send, but it is never Sync.
    // We need AssertUnwindSafe in order to keep this struct RefUnwindSafe.
    // We have a RefUnwindSafe-specific test lower down in this file.
    bar: AssertUnwindSafe<RefCell<usize>>,
}

#[cfg(not(feature = "auto_trait_impl_removed"))]
pub struct SendStruct {
    // RefCell<T> is Send if T is Send, but it is never Sync.
    bar: RefCell<usize>,
}

#[cfg(feature = "auto_trait_impl_removed")]
pub struct SendStruct {
    // This is just a silly type that is neither Send nor Sync.
    // It's not actually useful for anything.
    bar: RefCell<Rc<usize>>,
}

#[cfg(not(feature = "auto_trait_impl_removed"))]
pub struct UnwindSafeStruct<'a> {
    bar: &'a i64,
}

#[cfg(feature = "auto_trait_impl_removed")]
pub struct UnwindSafeStruct<'a> {
    // &mut T is not UnwindSafe unless T is AssertUnwindSafe.
    bar: &'a mut i64,
}

#[cfg(not(feature = "auto_trait_impl_removed"))]
pub struct RefUnwindSafeStruct {
    bar: Rc<i64>,
}

#[cfg(feature = "auto_trait_impl_removed")]
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
#[cfg(feature = "auto_trait_impl_removed")]
impl UnwindSafe for RefUnwindSafeStruct {}

#[cfg(not(feature = "auto_trait_impl_removed"))]
pub struct UnpinStruct {
    bar: i64,
}

#[cfg(feature = "auto_trait_impl_removed")]
pub struct UnpinStruct {
    bar: i64,
    _marker: PhantomPinned,
}
