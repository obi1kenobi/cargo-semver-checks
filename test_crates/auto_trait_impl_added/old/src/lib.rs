#![no_std]

use core::cell::RefCell;

// struct gains Sync and RefUnwindSafe
// when the RefCell private field is replaced
pub struct PubStruct {
    s: RefCell<usize>,
}
