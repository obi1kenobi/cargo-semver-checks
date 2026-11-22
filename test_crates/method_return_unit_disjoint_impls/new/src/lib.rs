#![no_std]

// Only the u8 method changes. Its u16 sibling still returns i32.
// TODO: Report the u8 method once its impl target can be matched across versions.
pub struct Partial<T>(pub T);
impl Partial<u8> {
    pub fn value() {}
}
impl Partial<u16> {
    pub fn value() -> i32 {
        8
    }
}

// Only `Converged::<u8>::value()` starts returning unit. The u16 method already returned unit.
// TODO: Report the u8 change once impl targets can be matched across versions.
pub struct Converged<T>(pub T);
impl Converged<u8> {
    pub fn value() {}
}
impl Converged<u16> {
    pub fn value() {}
}

// The u8 method disappears. The u16 method already returned unit.
pub struct RemovedOriginal<T>(pub T);
impl RemovedOriginal<u16> {
    pub fn value() {}
}

// Each version has exactly one method named `value`, on different instantiations.
pub struct ReplacedImpl<T>(pub T);
impl ReplacedImpl<u16> {
    pub fn value() {}
}

// The i32-returning method survives the addition of a unit-returning sibling.
pub struct Added<T>(pub T);
impl Added<u8> {
    pub fn value() -> i32 {
        8
    }
}
impl Added<u16> {
    pub fn value() {}
}

// The baseline method belongs to a nondefault type argument.
pub struct LosesParameter(core::marker::PhantomData<u8>);
impl LosesParameter {
    pub fn value(&self) {}
}

// The current method belongs to a nondefault const argument.
pub struct GainsParameter<const N: usize = 0>;
impl GainsParameter<1> {
    pub fn value(&self) {}
}

// Callers can still use `OwnerKindChanged::value()` after the struct becomes an enum.
// Report the loss of its u8 return value.
#[non_exhaustive]
pub enum OwnerKindChanged {
    Base,
}
impl OwnerKindChanged {
    pub fn value() {}
}

// The inherent function wins lookup even with Value in scope.
// Report the change to TraitShadowed::value().
pub struct TraitShadowed;
impl TraitShadowed {
    pub fn value() {}
}

pub trait Value {
    fn value() -> i32;
}
impl Value for TraitShadowed {
    fn value() -> i32 {
        8
    }
}

// Removing the inherent function leaves only a trait function that returns unit.
pub struct TraitOnly;
pub trait UnitValue {
    fn value();
}
impl UnitValue for TraitOnly {
    fn value() {}
}

// The variant shadows the qualified method name. On a mutable local, `value.Changed()`
// calls the method in both versions and loses its u8 return value.
pub enum DotLookup {
    Changed,
}

impl DotLookup {
    #[allow(non_snake_case)]
    pub fn Changed(&mut self) {}
}
