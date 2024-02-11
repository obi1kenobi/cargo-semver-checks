use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

// Object safe traits
trait RefTrait {
    fn by_ref(self: &Self) {}
}

trait MutRefTrait {
    fn by_ref_mut(self: &mut Self) {}
}

trait BoxTrait {
    fn by_box(self: Box<Self>) {}
}

trait RcTrait {
    fn by_rc(self: Rc<Self>) {}
}

trait ArcTrait {
    fn by_arc(self: Arc<Self>) {}
}

trait PinTrait {
    fn by_pin(self: Pin<&Self>) {}
}

trait LifetimeTrait {
    fn with_lifetime<'a>(self: &'a Self) {}
}

trait NestedPinTrait {
    fn nested_pin(self: Pin<Arc<Self>>) {}
}