use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

pub struct PublicStruct;

impl PublicStruct {
    pub fn self_ref_to_refmut(&self) {
        let _ = self;
    }

    pub fn box_ref_to_owned(self: &Box<Self>) {
        let _ = self;
    }

    pub fn rc_refmut_to_ref(self: &mut Rc<Self>) {
        let _ = self;
    }

    pub fn rcbox_refmut_to_owned(self: &mut Rc<Box<Self>>) {
        let _ = self;
    }

    pub fn pin_owned_to_ref(self: Pin<&mut Self>) {
        let _ = self;
    }

    pub fn self_owned_to_refmut(self) {
        let _ = self;
    }

    pub fn ref_self_to_box(&self) {
        let _ = self;
    }

    pub fn refmut_self_to_pin(&mut self) {
        let _ = self;
    }

    pub fn owned_self_to_rc(self) {
        let _ = self;
    }

    pub fn ref_to_owned_pin(&self) {
        let _ = self;
    }

    pub fn refmut_box_to_ref_rcbox(self: &mut Box<Self>) {
        let _ = self;
    }

    pub fn owned_rc_to_refmut_box(self: Rc<Self>) {
        let _ = self;
    }

    pub fn method_to_assoc_fn(&self) {
        let _ = self;
    }
}

pub enum PublicEnum {
    A,
}

impl PublicEnum {
    pub fn self_ref_to_refmut(&self) {
        let _ = self;
    }

    pub fn box_ref_to_owned(self: &Box<Self>) {
        let _ = self;
    }

    pub fn rc_refmut_to_ref(self: &mut Rc<Self>) {
        let _ = self;
    }

    pub fn rcbox_refmut_to_owned(self: &mut Rc<Box<Self>>) {
        let _ = self;
    }

    pub fn pin_owned_to_ref(self: Pin<&mut Self>) {
        let _ = self;
    }

    pub fn self_owned_to_refmut(self) {
        let _ = self;
    }

    pub fn ref_self_to_box(&self) {
        let _ = self;
    }

    pub fn refmut_self_to_pin(&mut self) {
        let _ = self;
    }

    pub fn owned_self_to_rc(self) {
        let _ = self;
    }

    pub fn ref_to_owned_pin(&self) {
        let _ = self;
    }

    pub fn refmut_box_to_ref_rcbox(self: &mut Box<Self>) {
        let _ = self;
    }

    pub fn owned_rc_to_refmut_box(self: Rc<Self>) {
        let _ = self;
    }

    pub fn method_to_assoc_fn(&mut self) {
        let _ = self;
    }
}

#[doc(hidden)]
pub struct HiddenStruct;

impl HiddenStruct {
    pub fn self_ref_to_refmut(&self) {
        let _ = self;
    }

    pub fn box_ref_to_owned(self: &Box<Self>) {
        let _ = self;
    }

    pub fn rc_refmut_to_ref(self: &mut Rc<Self>) {
        let _ = self;
    }

    pub fn rcbox_refmut_to_owned(self: &mut Rc<Box<Self>>) {
        let _ = self;
    }

    pub fn pin_owned_to_ref(self: Pin<&mut Self>) {
        let _ = self;
    }

    pub fn self_owned_to_refmut(self) {
        let _ = self;
    }

    pub fn ref_self_to_box(&self) {
        let _ = self;
    }

    pub fn refmut_self_to_pin(&mut self) {
        let _ = self;
    }

    pub fn owned_self_to_rc(self) {
        let _ = self;
    }

    pub fn ref_to_owned_pin(&self) {
        let _ = self;
    }

    pub fn refmut_box_to_ref_rcbox(self: &mut Box<Self>) {
        let _ = self;
    }

    pub fn owned_rc_to_refmut_box(self: Rc<Self>) {
        let _ = self;
    }

    pub fn method_to_assoc_fn(&self) {
        let _ = self;
    }
}

pub enum HiddenEnumMethods {
    A,
}

impl HiddenEnumMethods {
    #[doc(hidden)]
    pub fn self_ref_to_refmut(&self) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn box_ref_to_owned(self: &Box<Self>) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn rc_refmut_to_ref(self: &mut Rc<Self>) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn rcbox_refmut_to_owned(self: &mut Rc<Box<Self>>) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn pin_owned_to_ref(self: Pin<&mut Self>) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn self_owned_to_refmut(self) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn ref_self_to_box(&self) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn refmut_self_to_pin(&mut self) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn owned_self_to_rc(self) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn ref_to_owned_pin(&self) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn refmut_box_to_ref_rcbox(self: &mut Box<Self>) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn owned_rc_to_refmut_box(self: Rc<Self>) {
        let _ = self;
    }

    #[doc(hidden)]
    pub fn method_to_assoc_fn(&self) {
        let _ = self;
    }
}
