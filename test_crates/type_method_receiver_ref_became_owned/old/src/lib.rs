use std::boxed::Box;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

pub struct PublicStruct;

impl PublicStruct {
    pub fn method_with_self_ref(&self) {}

    pub fn method_with_boxed_self_ref(self: &Box<Self>) {}

    pub fn method_with_pinned_boxed_self_ref(self: &Pin<Box<Self>>) {}
    pub fn method_with_pinned_mut_self(self: &Pin<&mut Self>) {}
    pub fn method_with_pinned_self_ref(self: &Pin<&Self>) {}

    pub fn method_with_rc_self_ref(self: &Rc<Self>) {}
    pub fn method_with_rc_boxed_self_ref(self: &Rc<Box<Self>>) {}

    pub fn method_with_arc_self(self: &Arc<Self>) {}

    // these shouldn't trigger
    #[doc(hidden)]
    pub fn doc_hidden_method_with_self_ref(&self) {}

    pub fn method_with_mut_boxed_self_ref(self: &mut Box<Self>) {}

    pub fn method_with_boxed_self_ref_that_becomes_pinned(self: &Box<Self>) {} // changes kind

    pub fn method_with_self_ref_that_becomes_mut_ref(&self) {}
}

pub enum PublicEnum {
    A,
}

impl PublicEnum {
    pub fn method_with_self_ref(&self) {}

    pub fn method_with_boxed_self_ref(self: &Box<Self>) {}

    pub fn method_with_pinned_boxed_self_ref(self: &Pin<Box<Self>>) {}
    pub fn method_with_pinned_mut_self(self: &Pin<&mut Self>) {}
    pub fn method_with_pinned_self_ref(self: &Pin<&Self>) {}

    pub fn method_with_rc_self_ref(self: &Rc<Self>) {}
    pub fn method_with_rc_boxed_self_ref(self: &Rc<Box<Self>>) {}

    pub fn method_with_arc_self(self: &Arc<Self>) {}

    // these shouldn't trigger
    #[doc(hidden)]
    pub fn doc_hidden_method_with_self_ref(&self) {}

    pub fn method_with_mut_boxed_self_ref(self: &mut Box<Self>) {}

    pub fn method_with_boxed_self_ref_that_becomes_pinned(self: &Box<Self>) {} // changes kind

    pub fn method_with_self_ref_that_becomes_mut_ref(&self) {}
}

pub union PublicUnion {
    pub i: i32,
}

impl PublicUnion {
    pub fn method_with_self_ref(&self) {}

    pub fn method_with_boxed_self_ref(self: &Box<Self>) {}

    pub fn method_with_pinned_boxed_self_ref(self: &Pin<Box<Self>>) {}
    pub fn method_with_pinned_mut_self(self: &Pin<&mut Self>) {}
    pub fn method_with_pinned_self_ref(self: &Pin<&Self>) {}

    pub fn method_with_rc_self_ref(self: &Rc<Self>) {}
    pub fn method_with_rc_boxed_self_ref(self: &Rc<Box<Self>>) {}

    pub fn method_with_arc_self(self: &Arc<Self>) {}

    // these shouldn't trigger
    #[doc(hidden)]
    pub fn doc_hidden_method_with_self_ref(&self) {}

    pub fn method_with_mut_boxed_self_ref(self: &mut Box<Self>) {}

    pub fn method_with_boxed_self_ref_that_becomes_pinned(self: &Box<Self>) {} // changes kind

    pub fn method_with_self_ref_that_becomes_mut_ref(&self) {}
}
