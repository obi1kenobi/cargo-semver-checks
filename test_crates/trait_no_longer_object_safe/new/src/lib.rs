use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

// Non-object safe traits
trait RefTrait {
    fn by_ref(self) -> Self;
}

trait MutRefTrait {
    fn by_ref_mut<T>(self: &mut Self, t: T) -> T;
}

trait BoxTrait {
    fn by_box(self: Box<Self>) -> Self;
}

trait RcTrait {
    fn by_rc<T>(self: Rc<Self>, t: T) -> Rc<T>;
}

trait ArcTrait {
    fn by_arc(self: Arc<Self>, value: Self) -> Arc<Self>;
}

trait PinTrait {
    fn by_pin(self: Pin<&mut Self>) -> Pin<Box<Self>>;
}

trait LifetimeTrait {
    fn with_lifetime<'a, 'b>(self: &'a Self, other: &'b Self) -> &'a Self;
}

trait NestedPinTrait {
    fn nested_pin<T>(self: Pin<Arc<Self>>, t: T) -> Pin<Arc<T>>;
}