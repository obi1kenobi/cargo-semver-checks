#![no_std]

// Calls such as `Partial::<u8>::reorder::<u16, 0>()` break when the
// generic arguments exchange positions. Calls supplying the old number of type
// or const arguments also break, but the u16 methods retain those call forms.
// TODO: Report the u8 changes once impl targets can be matched. Until then,
// these remain false negatives; new-new comparisons must reject invented changes.
pub struct Partial<T>(pub T);
impl Partial<u8> {
    pub fn reorder<T, const N: usize>() {}
    pub fn type_count<T>() {}
    pub fn const_count<const N: usize>() {}
}
impl Partial<u16> {
    pub fn reorder<T, const N: usize>() {}
    pub fn type_count<T>() {}
    pub fn const_count<const N: usize>() {}
}

// Each impl adopts the other's generic argument order and counts. Explicit
// calls to both impls break although the available orders and counts are unchanged.
// TODO: Report these changes once impl targets can be matched. Both old-old
// and new-new comparisons must remain free of false positives.
pub struct Swapped<T>(pub T);
impl Swapped<u8> {
    pub fn reorder<T, const N: usize>() {}
    pub fn type_count<T>() {}
    pub fn const_count<const N: usize>() {}
}
impl Swapped<u16> {
    pub fn reorder<const N: usize, T>() {}
    pub fn type_count<T, U>() {}
    pub fn const_count<const N: usize, const M: usize>() {}
}

// Only the u8 methods change. Their old generic argument order and counts
// disappear entirely, so report their losses once each. The unchanged u16
// declarations must not be identified as the methods that changed.
pub struct Converged<T>(pub T);
impl Converged<u8> {
    pub fn reorder<T, const N: usize>() {}
    pub fn type_count<T>() {}
    pub fn const_count<const N: usize>() {}
}
impl Converged<u16> {
    pub fn reorder<const N: usize, T>() {}
    pub fn type_count<T, U>() {}
    pub fn const_count<const N: usize, const M: usize>() {}
}

// Both methods exchange their type and const arguments. Report each old
// declaration once per reordered position, rather than every pairing of methods.
pub struct BothChanged<T>(pub T);
impl BothChanged<u8> {
    pub fn reorder<T, const N: usize>() {}
}
impl BothChanged<u16> {
    pub fn reorder<T, const N: usize>() {}
}

// The u8 method stops accepting a type at position 1, and no method retains it.
// Report that loss without attributing the change to the unchanged u16 method.
// Position 2 still accepts a const on u16, although that cannot preserve u8 calls.
// TODO: Report the u8 const-position change once impl targets can be matched.
pub struct DistinctOrders<T>(pub T);
impl DistinctOrders<u8> {
    pub fn reorder<T, const N: usize, const M: usize, U>() {}
}
impl DistinctOrders<u16> {
    pub fn reorder<const N: usize, const M: usize, T, U>() {}
}

// Adding methods on Added<u16> preserves all existing calls on Added<u8>.
// The new methods' different generic argument order and counts are not changes
// to the old declarations.
pub struct Added<T>(pub T);
impl Added<u8> {
    pub fn reorder<T, const N: usize>() {}
    pub fn type_count<T>() {}
    pub fn const_count<const N: usize>() {}
}

// Both functions exchange the first type and const arguments, but each also
// changes one parameter count. Report these through the corresponding count
// lint; neither reordering lint should duplicate those reports.
pub struct CountChanged;
impl CountChanged {
    pub fn type_count<T, const N: usize>() {}
    pub fn const_count<T, const N: usize>() {}
}

// The u8 method leaves the supported API without changing generic parameters.
// Do not mistake the unchanged u16 method for its replacement.
// TODO: Attribute the API loss to u8 once impl targets can be matched.
pub struct HiddenOriginal<T>(pub T);
impl HiddenOriginal<u8> {
    pub fn reorder<T, const N: usize>() {}
}
impl HiddenOriginal<u16> {
    pub fn reorder<const N: usize, T>() {}
}

// The u8 method becomes private, while its generic parameters and the public
// u16 method stay unchanged. This is an API removal, not a parameter reorder.
// TODO: Attribute the API loss to u8 once impl targets can be matched.
pub struct PrivateOriginal<T>(pub T);
impl PrivateOriginal<u8> {
    pub fn reorder<T, const N: usize>() {}
}
impl PrivateOriginal<u16> {
    pub fn reorder<const N: usize, T>() {}
}

// The public u8 method exchanges type and const arguments while a hidden
// same-named u16 method retains the old order. Without impl matching,
// the hidden declaration could instead be the old method leaving the public API.
// TODO: Report the u8 reorder once impl targets can be matched.
pub struct HiddenSibling<T>(pub T);
impl HiddenSibling<u8> {
    pub fn reorder<T, const N: usize>() {}
}
impl HiddenSibling<u16> {
    #[doc(hidden)]
    pub fn reorder<T, const N: usize>() {}
}

// These traits exist in both versions, so the following changes do not
// require downstream users to add new imports.
pub trait OriginalOrder {
    fn reorder<T, const N: usize>();
}

pub trait Reordered {
    fn reorder<const N: usize, T>();

    // A different function's parameter order cannot preserve calls to `reorder`.
    fn unrelated<T, const N: usize>() {}
}

// Calls on TraitPreserved<u8> move to its existing trait implementation.
// An unrelated inherent method on u16 remains. Its different generic argument
// order must not be reported as a change to the u8 method.
pub struct TraitPreserved<T>(pub T);
impl TraitPreserved<u8> {
    pub fn reorder<T, const N: usize>() {}
}
impl TraitPreserved<u16> {
    pub fn reorder<const N: usize, T>() {}
}
impl OriginalOrder for TraitPreserved<u8> {
    fn reorder<T, const N: usize>() {}
}

// The inherent function changes from <T, N> to <N, T>. It shadows the trait
// function in both versions, so report each reordered position at most once.
// The old-old comparison must not diagnose the trait's already-different order.
pub struct ForwardShadowed;
impl ForwardShadowed {
    pub fn reorder<T, const N: usize>() {}
}
impl Reordered for ForwardShadowed {
    fn reorder<const N: usize, T>() {}
}

// The unchanged trait function has the old parameter order, but cannot preserve
// calls to ReverseShadowed::reorder because the changed inherent function shadows it.
// Report both reordered positions in the inherent function.
pub struct ReverseShadowed;
impl ReverseShadowed {
    pub fn reorder<T, const N: usize>() {}
}
impl OriginalOrder for ReverseShadowed {
    fn reorder<T, const N: usize>() {}
}

// With Reordered in scope, Fallback::reorder now resolves to the trait function.
// Its changed generic argument order breaks the old explicit call and must be reported.
pub struct Fallback;
impl Fallback {
    pub fn reorder<T, const N: usize>() {}
}

// A public doc-hidden inherent function still shadows the trait function.
// Its parameter order is unchanged, so report only its removal from the supported API.
pub struct HiddenShadow;
impl HiddenShadow {
    pub fn reorder<T, const N: usize>() {}
}
impl Reordered for HiddenShadow {
    fn reorder<const N: usize, T>() {}
}

// A private inherent function does not block the public trait function in an
// external call to PrivateShadow::reorder. With Reordered in scope, the new call
// uses the trait's different parameter order, so report both reordered positions.
pub struct PrivateShadow;
impl PrivateShadow {
    pub fn reorder<T, const N: usize>() {}
}
impl Reordered for PrivateShadow {
    fn reorder<const N: usize, T>() {}
}

// Only the u8 methods change their generic parameter counts; the u16 methods
// retain their existing counts and order. Explicit u8 calls break through count
// changes, so the unchanged u16 declarations must not be diagnosed as reordered.
// TODO: Report the u8 count changes once impl targets can be matched.
pub struct MixedCounts<T>(pub T);
impl MixedCounts<u8> {
    pub fn type_removed<T, const N: usize>() {}
    pub fn const_removed<const N: usize, T>() {}

    // An unrelated function's counts do not preserve the other functions' calls.
    pub fn unrelated<T, const N: usize>() {}
}
impl MixedCounts<u16> {
    pub fn type_removed<const N: usize, T>() {}
    pub fn const_removed<T, const N: usize>() {}
}

// Calls passing () remain valid for the value argument, but their explicit
// type and const arguments exchange positions. Report both reordered positions
// whether an `impl Trait` parameter is removed or added: its compiler-generated
// type parameter does not change the number of explicitly supplied arguments.
pub struct SyntheticReordered;
impl SyntheticReordered {
    pub fn remove_impl_trait<T, const N: usize>(_: impl Send) {}
    pub fn add_impl_trait<T, const N: usize>(_: ()) {}
}

// The number of explicit type arguments changes, while adding or removing
// `impl Trait` keeps the total number of type parameters unchanged.
// Report the explicit count change only, without duplicate reorder reports.
pub struct SyntheticCountChanged;
impl SyntheticCountChanged {
    pub fn remove_impl_trait<T, const N: usize>(_: impl Send) {}
    pub fn add_impl_trait<T, const N: usize, U>(_: ()) {}
}

// This unrelated method cannot affect the generic arguments accepted by
// the reordered Converged methods on u8 and u16.
impl Converged<u32> {
    pub fn unrelated() {}
}

// A doc-hidden trait function still preserves explicit calls
// on HiddenTraitPreserved<u8> after its inherent method is removed.
// Do not mistake the unchanged u16 method for a reordered replacement.
pub struct HiddenTraitPreserved<T>(pub T);
impl HiddenTraitPreserved<u8> {
    pub fn reorder<T, const N: usize>() {}
}
impl HiddenTraitPreserved<u16> {
    pub fn reorder<const N: usize, T>() {}
}
impl OriginalOrder for HiddenTraitPreserved<u8> {
    #[doc(hidden)]
    fn reorder<T, const N: usize>() {}
}

// Explicit type and const arguments exchange positions on u8.
// The trait's `reorder` already uses the new order and cannot preserve the old call.
// Report both reordered positions while ignoring the added `impl Trait`
// parameter when counting the method's explicitly supplied type arguments.
pub struct GenericSyntheticReordered<T>(pub T);
impl GenericSyntheticReordered<u8> {
    pub fn reorder<T, const N: usize>(_: ()) {}
}
impl Reordered for GenericSyntheticReordered<u8> {
    fn reorder<const N: usize, T>() {}
}

// These u8 methods gain a const or type argument, while the u16 methods
// keep the old counts. Their different order does not establish that the
// u8 methods were reordered. TODO: Report the count changes once impl targets
// can be matched, retaining the existing conservative count-lint behavior.
impl MixedCounts<u8> {
    pub fn const_added<T, const N: usize>() {}
    pub fn type_added<const N: usize, T>() {}
}
impl MixedCounts<u16> {
    pub fn const_added<const N: usize, T>() {}
    pub fn type_added<T, const N: usize>() {}
}

// An unrelated inherent function cannot prevent calls to Fallback::reorder
// from resolving to the trait function after its inherent declaration is removed.
impl Fallback {
    pub fn unrelated() {}
}

// Both traits are implemented in both versions. Importing only OriginalOrder
// preserves explicit calls after the inherent function disappears.
// The other trait's different parameter order must not be reported as a change.
pub struct NongenericTraitPreserved;
impl NongenericTraitPreserved {
    pub fn reorder<T, const N: usize>() {}
}
impl OriginalOrder for NongenericTraitPreserved {
    fn reorder<T, const N: usize>() {}
}
impl Reordered for NongenericTraitPreserved {
    fn reorder<const N: usize, T>() {}
}

// The doc-hidden implementation of OriginalOrder::reorder preserves explicit calls
// when the inherent function disappears and only OriginalOrder is imported.
// The public Reordered implementation must not cause a reorder report merely
// because the function preserving the old call is hidden from documentation.
pub struct NongenericHiddenTraitPreserved;
impl NongenericHiddenTraitPreserved {
    pub fn reorder<T, const N: usize>() {}
}
impl OriginalOrder for NongenericHiddenTraitPreserved {
    #[doc(hidden)]
    fn reorder<T, const N: usize>() {}
}
impl Reordered for NongenericHiddenTraitPreserved {
    fn reorder<const N: usize, T>() {}
}

// The u8 method becomes doc-hidden and exchanges its type and const arguments.
// The unchanged u16 method must not be mistaken for its public replacement.
// TODO: Report the u8 API loss once impl targets can be matched; until then,
// neither the hiding lint nor the reordering lints should report this change.
pub struct GenericHiddenReordered<T>(pub T);
impl GenericHiddenReordered<u8> {
    pub fn reorder<T, const N: usize>() {}
}
impl GenericHiddenReordered<u16> {
    pub fn reorder<const N: usize, T>() {}
}

// The inherent function's new parameter order breaks explicit calls, but it
// also becomes doc-hidden. Report only that API loss. Its public inherent
// declaration still shadows the trait function despite being hidden.
pub struct HiddenChangedShadow;
impl HiddenChangedShadow {
    pub fn reorder<T, const N: usize>() {}
}
impl Reordered for HiddenChangedShadow {
    fn reorder<const N: usize, T>() {}
}
