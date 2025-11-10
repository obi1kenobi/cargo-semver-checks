use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

pub trait PublicTrait {
    fn self_ref_to_refmut(&mut self);

    fn box_ref_to_owned(self: Box<Self>);

    fn rc_refmut_to_ref(self: &Rc<Self>);

    fn rcbox_refmut_to_owned(self: Rc<Box<Self>>);

    fn pin_owned_to_ref(self: &Pin<&mut Self>);

    fn self_owned_to_refmut(&mut self);

    fn ref_self_to_box(self: &Box<Self>);

    fn refmut_self_to_pin(self: &mut Pin<&mut Self>);

    fn owned_self_to_rc(self: Rc<Self>);

    fn ref_to_owned_pin(self: Pin<&mut Self>);

    fn refmut_box_to_ref_rcbox(self: &Rc<Box<Self>>);

    fn owned_rc_to_refmut_box(self: &mut Box<Self>);

    fn method_to_assoc_fn();

    fn assoc_fn_to_method(&self);
}

#[doc(hidden)]
pub trait HiddenTrait {
    fn self_ref_to_refmut(&mut self);

    fn box_ref_to_owned(self: Box<Self>);

    fn rc_refmut_to_ref(self: &Rc<Self>);

    fn rcbox_refmut_to_owned(self: Rc<Box<Self>>);

    fn pin_owned_to_ref(self: &Pin<&mut Self>);

    fn self_owned_to_refmut(&mut self);

    fn ref_self_to_box(self: &Box<Self>);

    fn refmut_self_to_pin(self: &mut Pin<&mut Self>);

    fn owned_self_to_rc(self: Rc<Self>);

    fn ref_to_owned_pin(self: Pin<&mut Self>);

    fn refmut_box_to_ref_rcbox(self: &Rc<Box<Self>>);

    fn owned_rc_to_refmut_box(self: &mut Box<Self>);

    fn method_to_assoc_fn();

    fn assoc_fn_to_method(&self);
}

pub trait HiddenMethods {
    #[doc(hidden)]
    fn self_ref_to_refmut(&mut self);

    #[doc(hidden)]
    fn box_ref_to_owned(self: Box<Self>);

    #[doc(hidden)]
    fn rc_refmut_to_ref(self: &Rc<Self>);

    #[doc(hidden)]
    fn rcbox_refmut_to_owned(self: Rc<Box<Self>>);

    #[doc(hidden)]
    fn pin_owned_to_ref(self: &Pin<&mut Self>);

    #[doc(hidden)]
    fn self_owned_to_refmut(&mut self);

    #[doc(hidden)]
    fn ref_self_to_box(self: &Box<Self>);

    #[doc(hidden)]
    fn refmut_self_to_pin(self: &mut Pin<&mut Self>);

    #[doc(hidden)]
    fn owned_self_to_rc(self: Rc<Self>);

    #[doc(hidden)]
    fn ref_to_owned_pin(self: Pin<&mut Self>);

    #[doc(hidden)]
    fn refmut_box_to_ref_rcbox(self: &Rc<Box<Self>>);

    #[doc(hidden)]
    fn owned_rc_to_refmut_box(self: &mut Box<Self>);

    #[doc(hidden)]
    fn method_to_assoc_fn();

    #[doc(hidden)]
    fn assoc_fn_to_method(&self);
}

pub trait PublicApiSealedTrait: SealSuper {
    fn assoc_fn_to_method(&self);
}

#[doc(hidden)]
pub trait HiddenPublicApiSealedTrait: SealSuper {
    fn assoc_fn_to_method(&self);
}

pub trait PublicApiSealedTraitWithHiddenMethods: SealSuper {
    #[doc(hidden)]
    fn hidden_assoc_fn_to_method(&self);
}

pub trait UnconditionallySealedTrait: private::Sealed {
    fn assoc_fn_to_method(&self);
}

#[doc(hidden)]
pub trait SealSuper {}

mod private {
    pub trait Sealed {}
}
