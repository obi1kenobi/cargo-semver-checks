use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

pub struct PublicStruct;

impl PublicStruct {
    pub fn self_ref_to_refmut(&mut self) {
        let _ = self;
    }

    pub fn box_ref_to_owned(self: Box<Self>) {
        let _ = self;
    }

    pub fn rc_refmut_to_ref(self: &Rc<Self>) {
        let _ = self;
    }

    pub fn rcbox_refmut_to_owned(self: Rc<Box<Self>>) {
        let _ = self;
    }

    pub fn pin_owned_to_ref(self: &Pin<&mut Self>) {
        let _ = self;
    }

    pub fn self_owned_to_refmut(&mut self) {
        let _ = self;
    }

    pub fn ref_self_to_box(self: &Box<Self>) {
        let _ = self;
    }

    pub fn refmut_self_to_pin(self: &mut Pin<&mut Self>) {
        let _ = self;
    }

    pub fn owned_self_to_rc(self: Rc<Self>) {
        let _ = self;
    }

    pub fn ref_to_owned_pin(self: Pin<&mut Self>) {
        let _ = self;
    }

    pub fn refmut_box_to_ref_rcbox(self: &Rc<Box<Self>>) {
        let _ = self;
    }

    pub fn owned_rc_to_refmut_box(self: &mut Box<Self>) {
        let _ = self;
    }

    pub fn method_to_assoc_fn() {}
}

pub enum PublicEnum {
    A,
}

impl PublicEnum {
    pub fn self_ref_to_refmut(&mut self) {
        let _ = self;
    }

    pub fn box_ref_to_owned(self: Box<Self>) {
        let _ = self;
    }

    pub fn rc_refmut_to_ref(self: &Rc<Self>) {
        let _ = self;
    }

    pub fn rcbox_refmut_to_owned(self: Rc<Box<Self>>) {
        let _ = self;
    }

    pub fn pin_owned_to_ref(self: &Pin<&mut Self>) {
        let _ = self;
    }

    pub fn self_owned_to_refmut(&mut self) {
        let _ = self;
    }

    pub fn ref_self_to_box(self: &Box<Self>) {
        let _ = self;
    }

    pub fn refmut_self_to_pin(self: &mut Pin<&mut Self>) {
        let _ = self;
    }

    pub fn owned_self_to_rc(self: Rc<Self>) {
        let _ = self;
    }

    pub fn ref_to_owned_pin(self: Pin<&mut Self>) {
        let _ = self;
    }

    pub fn refmut_box_to_ref_rcbox(self: &Rc<Box<Self>>) {
        let _ = self;
    }

    pub fn owned_rc_to_refmut_box(self: &mut Box<Self>) {
        let _ = self;
    }

    pub fn method_to_assoc_fn() {}
}

#[doc(hidden)]
pub struct HiddenStruct;

impl HiddenStruct {
    pub fn self_ref_to_refmut(&mut self) {
        let _ = self;
    }

    pub fn box_ref_to_owned(self: Box<Self>) {
        let _ = self;
    }

    pub fn rc_refmut_to_ref(self: &Rc<Self>) {
        let _ = self;
    }

    pub fn rcbox_refmut_to_owned(self: Rc<Box<Self>>) {
        let _ = self;
    }

    pub fn pin_owned_to_ref(self: &Pin<&mut Self>) {
        let _ = self;
    }

    pub fn self_owned_to_refmut(&mut self) {
        let _ = self;
    }

    pub fn ref_self_to_box(self: &Box<Self>) {
        let _ = self;
    }

    pub fn refmut_self_to_pin(self: &mut Pin<&mut Self>) {
        let _ = self;
    }

    pub fn owned_self_to_rc(self: Rc<Self>) {
        let _ = self;
    }

    pub fn ref_to_owned_pin(self: Pin<&mut Self>) {
        let _ = self;
    }

    pub fn refmut_box_to_ref_rcbox(self: &Rc<Box<Self>>) {
        let _ = self;
    }

    pub fn owned_rc_to_refmut_box(self: &mut Box<Self>) {
        let _ = self;
    }

    pub fn method_to_assoc_fn() {}
}

pub enum HiddenEnumMethods {
    A,
}

impl HiddenEnumMethods {
    #[doc(hidden)]
    pub fn self_ref_to_refmut(&mut self) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn box_ref_to_owned(self: Box<Self>) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn rc_refmut_to_ref(self: &Rc<Self>) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn rcbox_refmut_to_owned(self: Rc<Box<Self>>) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn pin_owned_to_ref(self: &Pin<&mut Self>) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn self_owned_to_refmut(&mut self) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn ref_self_to_box(self: &Box<Self>) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn refmut_self_to_pin(self: &mut Pin<&mut Self>) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn owned_self_to_rc(self: Rc<Self>) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn ref_to_owned_pin(self: Pin<&mut Self>) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn refmut_box_to_ref_rcbox(self: &Rc<Box<Self>>) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn owned_rc_to_refmut_box(self: &mut Box<Self>) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn method_to_assoc_fn() {}
}

pub struct EqualLifetimes<'a: 'b, 'b: 'a>(pub &'a (), pub &'b ());

impl<'a: 'b, 'b: 'a> EqualLifetimes<'a, 'b> {
    // False positive. The baseline receiver `&EqualLifetimes<'b, 'a>` has the
    // same type as `&Self` because each lifetime outlives the other.
    pub fn read(&self) {}
}

pub trait Identity {
    type This;
}

impl<T> Identity for T {
    type This = T;
}

pub struct ProjectedReceiver<T>(pub T);

impl<T> ProjectedReceiver<T> {
    // False positive. The blanket `Identity` impl sets `This` to the implementing
    // type, so the baseline receiver `&<Self as Identity>::This` is `&Self`.
    pub fn read(&self) {}
}
