#![no_std]

extern crate alloc;
use alloc::{boxed::Box, rc::Rc};

// Each u8 method gains the receiver form previously used by its u16 counterpart.
// This breaks immutable borrows, repeated borrowed calls, or calls without a Box.
// TODO: Report the changed methods once we can match impls by implementing type.
// Until then, the retained receiver forms make these changes ambiguous. Both
// old-old and new-new comparisons must still reject invented receiver changes.
pub struct Swapped<T>(pub T);
impl Swapped<u8> {
    pub fn to_mut(&self) {}
    pub fn ref_to_owned(&self) {}
    pub fn mut_to_owned(&mut self) {}
    pub fn different_kind(&self) {}
}
impl Swapped<u16> {
    pub fn to_mut(&mut self) {}
    pub fn ref_to_owned(self) {}
    pub fn mut_to_owned(self) {}
    pub fn different_kind(self: Box<Self>) {}
}

// Only the u8 methods change; u16 still supports the original calls. Those methods
// cannot preserve calls on u8, but their impl targets are not yet distinguishable.
// TODO: Report these genuine receiver changes, including receiver removal, once
// impl matching is available. The new-new comparison exercises the false positives.
pub struct Partial<T>(pub T);
impl Partial<u8> {
    pub fn to_mut(&self) {}
    pub fn ref_to_owned(&self) {}
    pub fn mut_to_owned(&mut self) {}
    pub fn different_kind(&self) {}
}
impl Partial<u16> {
    pub fn to_mut(&self) {}
    pub fn ref_to_owned(&self) {}
    pub fn mut_to_owned(&mut self) {}
    pub fn different_kind(&self) {}
}
impl Partial<u8> {
    pub fn removed(&self) {}
}
impl Partial<u16> {
    pub fn removed(&self) {}
}

// Only the u8 methods change. The u16 methods already had the new receivers.
// TODO: Report the four changes once we can match each u8 method to its current
// declaration. In RemovedOriginal the u8 methods disappear instead.
pub struct Converged<T>(pub T);
impl Converged<u8> {
    pub fn to_mut(&self) {}
    pub fn ref_to_owned(&self) {}
    pub fn mut_to_owned(&mut self) {}
    pub fn different_kind(&self) {}
}
impl Converged<u16> {
    pub fn to_mut(&mut self) {}
    pub fn ref_to_owned(self) {}
    pub fn mut_to_owned(self) {}
    pub fn different_kind(self: Box<Self>) {}
}

// Both methods now require a mutable borrow.
// TODO: Report each change once we can match its impl across versions.
pub struct BothChanged<T>(pub T);
impl BothChanged<u8> {
    pub fn read(&self) {}
}
impl BothChanged<u16> {
    pub fn read(&self) {}
}

// The u8 methods change, but several different receiver forms remain in use.
// TODO: Identify their replacements once impl matching is available. Until then,
// do not choose between conflicting mutable/owned receiver diagnoses.
pub struct MixedDestinations<T>(pub T);
impl MixedDestinations<u8> {
    pub fn to_mut(&self) {}
    pub fn ref_to_owned(&self) {}
    pub fn mut_to_owned(&mut self) {}
}
impl MixedDestinations<u16> {
    pub fn to_mut(self) {}
    pub fn ref_to_owned(&mut self) {}
    pub fn mut_to_owned(&self) {}
}

// Calls on &DifferentKinds<u8> break when its method starts requiring a Box.
// TODO: Report the change to Box<Self> once we can distinguish the u8 method
// from the unchanged u16 method that takes Rc<Self>.
pub struct DifferentKinds<T>(pub T);
impl DifferentKinds<u8> {
    pub fn read(&self) {}
}
impl DifferentKinds<u16> {
    pub fn read(self: Rc<Self>) {}
}

// Adding methods on Added<u16> does not change existing calls on Added<u8>.
pub struct Added<T>(pub T);
impl Added<u8> {
    pub fn to_mut(&self) {}
    pub fn ref_to_owned(&self) {}
    pub fn mut_to_owned(&mut self) {}
    pub fn different_kind(&self) {}
}

// The u8 methods lose their receivers; u16 retains unrelated receiver forms.
// TODO: Detect the u8 removals once impl targets can be matched. Do not misreport
// them as conversions to the receivers of the unchanged u16 methods.
pub struct ReceiverlessAlternative<T>(pub T);
impl ReceiverlessAlternative<u8> {
    pub fn to_mut(&self) {}
    pub fn ref_to_owned(&self) {}
    pub fn mut_to_owned(&mut self) {}
    pub fn different_kind(&self) {}
}
impl ReceiverlessAlternative<u16> {
    pub fn to_mut(&mut self) {}
    pub fn ref_to_owned(self) {}
    pub fn mut_to_owned(self) {}
    pub fn different_kind(self: Box<Self>) {}
}

// Relaxing a mutable borrow to a shared borrow does not transfer ownership.
// The unchanged owned receiver on u16 must not be mistaken for the new u8 method.
pub struct BorrowRelaxed<T>(pub T);
impl BorrowRelaxed<u8> {
    pub fn read(&mut self) {}
}
impl BorrowRelaxed<u16> {
    pub fn read(self) {}
}

// The u8 methods leave the supported API without changing receivers. The public
// u16 methods are unchanged. Do not diagnose their receiver forms as replacements.
// TODO: Detect the u8 API loss once impl targets can be matched.
pub struct HiddenOriginal<T>(pub T);
impl HiddenOriginal<u8> {
    pub fn to_mut(&self) {}
    pub fn ref_to_owned(&self) {}
    pub fn mut_to_owned(&mut self) {}
    pub fn different_kind(&self) {}
}
impl HiddenOriginal<u16> {
    pub fn to_mut(&mut self) {}
    pub fn ref_to_owned(self) {}
    pub fn mut_to_owned(self) {}
    pub fn different_kind(self: Box<Self>) {}
}

// The u8 methods leave the supported API without changing receivers. The public
// u16 methods are unchanged. Do not diagnose their receiver forms as replacements.
// TODO: Detect the u8 API loss once impl targets can be matched.
pub struct PrivateOriginal<T>(pub T);
impl PrivateOriginal<u8> {
    pub fn to_mut(&self) {}
    pub fn ref_to_owned(&self) {}
    pub fn mut_to_owned(&mut self) {}
    pub fn different_kind(&self) {}
}
impl PrivateOriginal<u16> {
    pub fn to_mut(&mut self) {}
    pub fn ref_to_owned(self) {}
    pub fn mut_to_owned(self) {}
    pub fn different_kind(self: Box<Self>) {}
}

// The u8 methods genuinely change receivers, while hidden same-named declarations
// remain on u16. TODO: Report u8 once impl targets can be matched. Until then, those
// hidden declarations could be the original methods with only visibility changed.
pub struct HiddenSibling<T>(pub T);
impl HiddenSibling<u8> {
    pub fn to_mut(&self) {}
    pub fn ref_to_owned(&self) {}
    pub fn mut_to_owned(&mut self) {}
    pub fn different_kind(&self) {}
}
impl HiddenSibling<u16> {
    #[doc(hidden)]
    pub fn to_mut(&self) {}
    #[doc(hidden)]
    pub fn ref_to_owned(&self) {}
    #[doc(hidden)]
    pub fn mut_to_owned(&mut self) {}
    #[doc(hidden)]
    pub fn different_kind(&self) {}
}

// A trait can preserve calls when TraitPreserved<u8> loses its inherent methods.
// Both crate versions provide this trait so downstream imports need not change.
pub trait OriginalReceivers {
    fn to_mut(&self);
    fn ref_to_owned(&self);
    fn mut_to_owned(&mut self);
    fn different_kind(&self);
}

pub struct TraitPreserved<T>(pub T);
impl TraitPreserved<u8> {
    pub fn to_mut(&self) {}
    pub fn ref_to_owned(&self) {}
    pub fn mut_to_owned(&mut self) {}
    pub fn different_kind(&self) {}
}
impl TraitPreserved<u16> {
    pub fn to_mut(&mut self) {}
    pub fn ref_to_owned(self) {}
    pub fn mut_to_owned(self) {}
    pub fn different_kind(self: Box<Self>) {}
}
impl OriginalReceivers for TraitPreserved<u8> {
    fn to_mut(&self) {}
    fn ref_to_owned(&self) {}
    fn mut_to_owned(&mut self) {}
    fn different_kind(&self) {}
}

// On a nongeneric owner, the changed inherent declarations are unambiguous.
// The trait's old receivers cannot preserve calls through these inherent functions.
pub struct TraitAlongside;
impl TraitAlongside {
    pub fn to_mut(&self) {}
    pub fn ref_to_owned(&self) {}
    pub fn mut_to_owned(&mut self) {}
    pub fn different_kind(&self) {}
}
impl OriginalReceivers for TraitAlongside {
    fn to_mut(&self) {}
    fn ref_to_owned(&self) {}
    fn mut_to_owned(&mut self) {}
    fn different_kind(&self) {}
}

// These methods also leave the supported API. Report that loss through the
// doc-hidden lint, rather than diagnosing the receivers of non-API methods.
pub struct HiddenDestination;
impl HiddenDestination {
    pub fn to_mut(&self) {}
    pub fn ref_to_owned(&self) {}
    pub fn mut_to_owned(&mut self) {}
    pub fn different_kind(&self) {}
}

// These changes also replace Self with Box<Self>. Only the receiver-type lint
// should report them, not a same-type borrowing or ownership transition.
pub struct DifferentKindBorrowing;
impl DifferentKindBorrowing {
    pub fn ref_to_mut_box(&self) {}
    pub fn mut_to_owned_box(&mut self) {}
}

// Calls on RemovedOriginal<u8> lose their methods. Each u16 method keeps its receiver.
// TODO: Report the u8 removals once impl targets can be matched.
pub struct RemovedOriginal<T>(pub T);
impl RemovedOriginal<u8> {
    pub fn to_mut(&self) {}
    pub fn ref_to_owned(&self) {}
    pub fn mut_to_owned(&mut self) {}
    pub fn different_kind(&self) {}
}
impl RemovedOriginal<u16> {
    pub fn to_mut(&mut self) {}
    pub fn ref_to_owned(self) {}
    pub fn mut_to_owned(self) {}
    pub fn different_kind(self: Box<Self>) {}
}

// Each name appears once per version. The u8 methods disappear and the u16 methods
// are new, so no caller sees a receiver change on the same instantiation.
pub struct ReplacedImpl<T>(pub T);
impl ReplacedImpl<u8> {
    pub fn to_mut(&self) {}
    pub fn ref_to_owned(&self) {}
    pub fn mut_to_owned(&mut self) {}
    pub fn different_kind(&self) {}
}

// The old methods require LosesParameter<u16>. Callers using the default type
// argument u8 gain these names when LosesParameter loses its parameter.
pub struct LosesParameter<T = u8>(core::marker::PhantomData<T>);
impl LosesParameter<u16> {
    pub fn to_mut(&self) {}
    pub fn ref_to_owned(&self) {}
    pub fn mut_to_owned(&mut self) {}
    pub fn different_kind(&self) {}
}

// With the default const argument 0, the old calls lose their methods.
// Only GainsParameter<1> provides the new methods.
pub struct GainsParameter;
impl GainsParameter {
    pub fn to_mut(&self) {}
    pub fn ref_to_owned(&self) {}
    pub fn mut_to_owned(&mut self) {}
    pub fn different_kind(&self) {}
}

// BorrowedBox::read(&boxed) still compiles by deref coercion. Its function pointer
// changes from fn(&Box<BorrowedBox>) to fn(&BorrowedBox).
pub struct BorrowedBox;
impl BorrowedBox {
    pub fn read(self: &Box<Self>) {}
}
