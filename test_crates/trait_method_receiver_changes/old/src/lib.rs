use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

pub trait PublicTrait {
    fn self_ref_to_refmut(&self);

    fn box_ref_to_owned(self: &Box<Self>);

    fn rc_refmut_to_ref(self: &mut Rc<Self>);

    fn rcbox_refmut_to_owned(self: &mut Rc<Box<Self>>);

    fn pin_owned_to_ref(self: Pin<&mut Self>);

    fn self_owned_to_refmut(self);

    fn ref_self_to_box(&self);

    fn refmut_self_to_pin(&mut self);

    fn owned_self_to_rc(self);

    fn ref_to_owned_pin(&self);

    fn refmut_box_to_ref_rcbox(self: &mut Box<Self>);

    fn owned_rc_to_refmut_box(self: Rc<Self>);

    fn method_to_assoc_fn(&self);

    fn assoc_fn_to_method();
}

#[doc(hidden)]
pub trait HiddenTrait {
    fn self_ref_to_refmut(&self);

    fn box_ref_to_owned(self: &Box<Self>);

    fn rc_refmut_to_ref(self: &mut Rc<Self>);

    fn rcbox_refmut_to_owned(self: &mut Rc<Box<Self>>);

    fn pin_owned_to_ref(self: Pin<&mut Self>);

    fn self_owned_to_refmut(self);

    fn ref_self_to_box(&self);

    fn refmut_self_to_pin(&mut self);

    fn owned_self_to_rc(self);

    fn ref_to_owned_pin(&self);

    fn refmut_box_to_ref_rcbox(self: &mut Box<Self>);

    fn owned_rc_to_refmut_box(self: Rc<Self>);

    fn method_to_assoc_fn(&self);

    fn assoc_fn_to_method();
}

pub trait HiddenMethods {
    #[doc(hidden)]
    fn self_ref_to_refmut(&self);

    #[doc(hidden)]
    fn box_ref_to_owned(self: &Box<Self>);

    #[doc(hidden)]
    fn rc_refmut_to_ref(self: &mut Rc<Self>);

    #[doc(hidden)]
    fn rcbox_refmut_to_owned(self: &mut Rc<Box<Self>>);

    #[doc(hidden)]
    fn pin_owned_to_ref(self: Pin<&mut Self>);

    #[doc(hidden)]
    fn self_owned_to_refmut(self);

    #[doc(hidden)]
    fn ref_self_to_box(&self);

    #[doc(hidden)]
    fn refmut_self_to_pin(&mut self);

    #[doc(hidden)]
    fn owned_self_to_rc(self);

    #[doc(hidden)]
    fn ref_to_owned_pin(&self);

    #[doc(hidden)]
    fn refmut_box_to_ref_rcbox(self: &mut Box<Self>);

    #[doc(hidden)]
    fn owned_rc_to_refmut_box(self: Rc<Self>);

    #[doc(hidden)]
    fn method_to_assoc_fn(&self);

    #[doc(hidden)]
    fn assoc_fn_to_method();
}

pub trait PublicApiSealedTrait: SealSuper {
    fn assoc_fn_to_method();
}

#[doc(hidden)]
pub trait HiddenPublicApiSealedTrait: SealSuper {
    fn assoc_fn_to_method();
}

pub trait PublicApiSealedTraitWithHiddenMethods: SealSuper {
    #[doc(hidden)]
    fn hidden_assoc_fn_to_method();
}

pub trait UnconditionallySealedTrait: private::Sealed {
    fn assoc_fn_to_method();
}

#[doc(hidden)]
pub trait SealSuper {}

mod private {
    pub trait Sealed {}
}
