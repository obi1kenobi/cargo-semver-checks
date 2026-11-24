#![no_std]

// `Exact::value(8)` gains const support. The method takes a type parameter on a nongeneric owner.
pub struct Exact;
impl Exact {
    pub fn value<T>(value: T) -> T {
        value
    }
}

// TODO: Report both const additions once impl targets can be matched. Each instantiation
// has its own `value` declaration.
pub struct BothChanged<T>(pub T);
impl BothChanged<u8> {
    pub fn value() -> i32 {
        8
    }
}
impl BothChanged<u16> {
    pub fn value() -> i32 {
        8
    }
}

// `Partial::<u8>::value()` gains const evaluation; the u16 function remains nonconst.
// TODO: Report only the u8 addition once impl targets can be matched.
pub struct Partial<T>(pub T);
impl Partial<u8> {
    pub fn value() -> i32 {
        8
    }
}
impl Partial<u16> {
    pub fn value() -> i32 {
        8
    }
}

// Const calls become valid on u8 and stop compiling on u16. Both const states remain
// present among same-named functions, so neither change can be assigned to an impl.
// TODO: Report the u8 addition and the u16 const removal once impl targets can be matched.
pub struct Swapped<T>(pub T);
impl Swapped<u8> {
    pub fn value() -> i32 {
        8
    }
}
impl Swapped<u16> {
    pub const fn value() -> i32 {
        8
    }
}

// Only `Converged::<u8>::value()` gains const evaluation. The baseline const u16 function
// prevents proving which current function gained const without matching impl targets.
// TODO: Report the u8 addition once impl targets can be matched.
pub struct Converged<T>(pub T);
impl Converged<u8> {
    pub fn value() -> i32 {
        8
    }
}
impl Converged<u16> {
    pub const fn value() -> i32 {
        8
    }
}

// Calls that explicitly select `Added<u8>` remain nonconst. A new const function on u16
// must not be reported as an existing function becoming const.
pub struct Added<T>(pub T);
impl Added<u8> {
    pub fn value() -> i32 {
        8
    }
}

// Removing the nonconst u8 function leaves the existing const u16 function. No function
// gains const evaluation, and the survivor must not be diagnosed as having changed.
// TODO: Report the u8 method removal once impl targets can be matched.
pub struct RemovedOriginal<T>(pub T);
impl RemovedOriginal<u8> {
    pub fn value() -> i32 {
        8
    }
}
impl RemovedOriginal<u16> {
    pub const fn value() -> i32 {
        8
    }
}

// TODO: Report the u16 const addition once impl targets can be matched.
// The u8 function becomes doc-hidden in the same release.
pub struct HiddenOriginal<T>(pub T);
impl HiddenOriginal<u8> {
    pub fn value() -> i32 {
        8
    }
}
impl HiddenOriginal<u16> {
    pub fn value() -> i32 {
        8
    }
}

// The inherent function gains const support even if the caller has imported `Value`.
pub struct TraitShadowed;
impl TraitShadowed {
    pub fn value() -> i32 {
        8
    }
}

// The u8 function disappears. The new const function belongs to u16,
// so no existing method became const.
pub struct ReplacedImpl<T>(pub T);
impl ReplacedImpl<u8> {
    pub fn value() -> i32 {
        8
    }
}

// The const lint compares methods only when their owner keeps its kind.
pub struct OwnerKindChanged {
    private: (),
}
impl OwnerKindChanged {
    pub fn value() -> i32 {
        8
    }
}

// The baseline method exists only on the nondefault type argument.
pub struct LosesParameter<T = u8>(core::marker::PhantomData<T>);
impl LosesParameter<u16> {
    pub fn value(&self) {}
}

// The current method exists only on the nondefault const argument.
pub struct GainsParameter;
impl GainsParameter {
    pub fn value(&self) {}
}

pub trait Value {
    fn value() -> i32;
}

impl Value for TraitShadowed {
    fn value() -> i32 {
        8
    }
}

// The variant shadows the qualified method name. On a mutable local, `value.Changed()`
// calls the method in both versions and gains const support.
pub enum DotLookup {
    Changed,
}

impl DotLookup {
    #[allow(non_snake_case)]
    pub fn Changed(&self) -> u8 {
        8
    }
}

pub struct Reader<T>(pub T);

impl Reader<u8> {
    pub const fn partial(&self) {}
    pub const fn converged(&self) {}
    pub const fn both_changed(&self) {}

    pub const fn removed(&self) {}
    pub const fn hidden(&self) {}
    pub const fn private(&self) {}
    pub const fn moved(&self) {}
}

impl Reader<u16> {
    pub const fn partial(&self) {}
    pub fn converged(&self) {}
    pub const fn both_changed(&self) {}

    pub fn removed(&self) {}
    pub fn hidden(&self) {}
    pub fn private(&self) {}
}

impl LosesParameter<u16> {
    pub const fn read() {}
}

impl GainsParameter {
    pub const fn read() {}
}

impl Exact {
    pub const fn identity<T>(x: T) -> T {
        x
    }
}
