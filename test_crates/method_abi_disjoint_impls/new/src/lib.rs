#![no_std]

// Only the u8 methods change ABI. Assigning them to function pointers with
// their old ABIs breaks, although the u16 methods retain those ABIs.
// TODO: Report the u8 changes once impl targets can be matched. Until then,
// the retained ABI contracts make these changes ambiguous.
pub struct Partial<T>(pub T);
impl Partial<u8> {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}
impl Partial<u16> {
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
    pub extern "C-unwind" fn no_longer_unwind() {}
}

// Each impl adopts the other's ABIs, breaking assignments to the old function pointer types.
// Across both impls, the set of available ABI contracts remains the same.
// TODO: Report the inherent-method changes once impl targets can be matched.
//
// The fixed export symbols identify the separate FFI contracts precisely, so
// exported_function_changed_abi, exported_function_abi_now_unwind, and
// exported_function_abi_no_longer_unwind should each report two ABI changes.
pub struct Swapped<T>(pub T);
impl Swapped<u8> {
    #[unsafe(export_name = "disjoint_abi_swapped_u8_changed_abi")]
    pub extern "Rust" fn changed_abi() {}
    #[unsafe(export_name = "disjoint_abi_swapped_u8_now_unwind")]
    pub extern "C-unwind" fn now_unwind() {}
    #[unsafe(export_name = "disjoint_abi_swapped_u8_no_longer_unwind")]
    pub extern "C" fn no_longer_unwind() {}
}
impl Swapped<u16> {
    #[unsafe(export_name = "disjoint_abi_swapped_u16_changed_abi")]
    pub extern "C" fn changed_abi() {}
    #[unsafe(export_name = "disjoint_abi_swapped_u16_now_unwind")]
    pub extern "C" fn now_unwind() {}
    #[unsafe(export_name = "disjoint_abi_swapped_u16_no_longer_unwind")]
    pub extern "C-unwind" fn no_longer_unwind() {}
}

// Only the u8 methods change ABI. Each now matches its unchanged u16 sibling.
// Removing the u8 methods instead would also leave the u16 ABIs available.
// TODO: Report the three changes on u8 once impl targets can be matched.
pub struct Converged<T>(pub T);
impl Converged<u8> {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}
impl Converged<u16> {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}

// Both impls change each method's ABI.
// TODO: Report each change on its own declaration once impl targets can be matched.
pub struct BothChanged<T>(pub T);
impl BothChanged<u8> {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}
impl BothChanged<u16> {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}

// Only the u8 methods change ABI. The u16 methods keep their system ABIs.
// TODO: Report the changes on u8 once impl targets can be matched, even when
// the unchanged u16 method has the same unwind behavior as the old u8 method.
pub struct ThirdAbi<T>(pub T);
impl ThirdAbi<u8> {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}
impl ThirdAbi<u16> {
    pub extern "system" fn changed_abi() {}
    pub extern "system" fn now_unwind() {}
    pub extern "system-unwind" fn no_longer_unwind() {}
}

// Calls and function pointer assignments that explicitly select `Added<u8>`
// retain their ABI contracts when methods are added on `Added<u16>`.
pub struct Added<T>(pub T);
impl Added<u8> {
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
    pub extern "C-unwind" fn no_longer_unwind() {}
}
impl Added<u16> {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}

// The u8 methods leave the supported API without changing their ABIs.
// Do not mistake the unchanged u16 methods for their replacements.
// TODO: Attribute the API loss to u8 once impl targets can be matched.
pub struct HiddenOriginal<T>(pub T);
impl HiddenOriginal<u8> {
    #[doc(hidden)]
    pub extern "C" fn changed_abi() {}
    #[doc(hidden)]
    pub extern "C" fn now_unwind() {}
    #[doc(hidden)]
    pub extern "C-unwind" fn no_longer_unwind() {}
}
impl HiddenOriginal<u16> {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}

// The u8 methods become private without changing their ABIs.
// The public u16 methods are unchanged, so their ABIs must not be diagnosed as replacements.
// TODO: Attribute the API loss to u8 once impl targets can be matched.
pub struct PrivateOriginal<T>(pub T);
impl PrivateOriginal<u8> {
    extern "C" fn changed_abi() {}
    extern "C" fn now_unwind() {}
    extern "C-unwind" fn no_longer_unwind() {}
}
impl PrivateOriginal<u16> {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}

// The public u8 methods change ABI while hidden methods retain the old ABIs.
// Without impl matching, the hidden declarations could instead be the old methods
// leaving the supported API. TODO: Report the u8 changes once targets can be matched.
pub struct HiddenSibling<T>(pub T);
impl HiddenSibling<u8> {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}
impl HiddenSibling<u16> {
    #[doc(hidden)]
    pub extern "C" fn changed_abi() {}
    #[doc(hidden)]
    pub extern "C" fn now_unwind() {}
    #[doc(hidden)]
    pub extern "C-unwind" fn no_longer_unwind() {}
}

// The public u8 methods change ABI while private methods retain the old ABIs.
// Without impl matching, those private declarations could instead be the old methods
// leaving the public API. TODO: Report the u8 changes once targets can be matched.
pub struct PrivateSibling<T>(pub T);
impl PrivateSibling<u8> {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}
impl PrivateSibling<u16> {
    extern "C" fn changed_abi() {}
    extern "C" fn now_unwind() {}
    extern "C-unwind" fn no_longer_unwind() {}
}

// Only hidden and private methods change ABI. The public u8 methods retain a different ABI.
// No hidden or private declaration should be reported.
pub struct ExcludedSources<T>(pub T);
impl ExcludedSources<u8> {
    pub extern "system" fn changed_abi() {}
    pub extern "system" fn now_unwind() {}
    pub extern "system-unwind" fn no_longer_unwind() {}
}
impl ExcludedSources<u16> {
    #[doc(hidden)]
    pub extern "Rust" fn changed_abi() {}
    #[doc(hidden)]
    pub extern "C-unwind" fn now_unwind() {}
    #[doc(hidden)]
    pub extern "C" fn no_longer_unwind() {}
}
impl ExcludedSources<u32> {
    extern "Rust" fn changed_abi() {}
    extern "C-unwind" fn now_unwind() {}
    extern "C" fn no_longer_unwind() {}
}

// These traits exist in both versions. The fallback cases assume
// the relevant trait is already in scope at the call site.
pub trait OriginalAbi {
    extern "C" fn changed_abi();
    extern "C" fn now_unwind();
    extern "C-unwind" fn no_longer_unwind();
}

pub trait ChangedAbi {
    extern "Rust" fn changed_abi();
    extern "C-unwind" fn now_unwind();
    extern "C" fn no_longer_unwind();
}

// With OriginalAbi in scope, calls on u8 move to its existing trait impl.
// Its doc-hidden methods remain callable with the same ABI contracts. The unrelated
// u16 inherent methods must not be reported as replacements with different ABIs.
pub struct TraitPreserved<T>(pub T);
impl TraitPreserved<u16> {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}
impl OriginalAbi for TraitPreserved<u8> {
    #[doc(hidden)]
    extern "C" fn changed_abi() {}
    #[doc(hidden)]
    extern "C" fn now_unwind() {}
    #[doc(hidden)]
    extern "C-unwind" fn no_longer_unwind() {}
}

// The changed inherent functions still shadow the unchanged trait functions.
// Their old function pointer assignments break despite the trait's retained ABIs,
// so report the inherent changes.
pub struct TraitShadowed;
impl TraitShadowed {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}
impl OriginalAbi for TraitShadowed {
    extern "C" fn changed_abi() {}
    extern "C" fn now_unwind() {}
    extern "C-unwind" fn no_longer_unwind() {}
}

// Removing the inherent functions exposes the trait functions' different ABIs.
// Assignments using the old function pointer types break with ChangedAbi in scope.
// TODO: Diagnose these trait fallback ABI changes when method resolution is modeled;
// the inherent-method ABI lints currently require an inherent replacement.
pub struct TraitFallback;
impl ChangedAbi for TraitFallback {
    extern "Rust" fn changed_abi() {}
    extern "C-unwind" fn now_unwind() {}
    extern "C" fn no_longer_unwind() {}
}

// The u8 methods leave the supported API and also change ABI. The unchanged
// u16 methods cannot establish replacements for them merely by sharing a name.
// TODO: Attribute the u8 changes once impl targets can be matched.
pub struct HiddenChanged<T>(pub T);
impl HiddenChanged<u8> {
    #[doc(hidden)]
    pub extern "Rust" fn changed_abi() {}
    #[doc(hidden)]
    pub extern "C-unwind" fn now_unwind() {}
    #[doc(hidden)]
    pub extern "C" fn no_longer_unwind() {}

    // A different public function cannot preserve the changed methods' API status.
    pub fn unrelated() {}
}
impl HiddenChanged<u16> {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}

// The u8 methods leave the supported API and also change ABI. The unchanged
// u16 methods cannot establish replacements for them merely by sharing a name.
// TODO: Attribute the u8 changes once impl targets can be matched.
pub struct PrivateChanged<T>(pub T);
impl PrivateChanged<u8> {
    extern "Rust" fn changed_abi() {}
    extern "C-unwind" fn now_unwind() {}
    extern "C" fn no_longer_unwind() {}
}
impl PrivateChanged<u16> {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}

// An async function's pointer type includes its ABI. Each change breaks code
// expecting a function pointer with the old ABI and a return type implementing `Future`.
pub struct AsyncAbi;
impl AsyncAbi {
    pub async extern "Rust" fn changed_abi() {}
    #[expect(
        improper_ctypes_definitions,
        reason = "The async ABI contract includes an opaque future return type."
    )]
    pub async extern "C-unwind" fn now_unwind() {}
    #[expect(
        improper_ctypes_definitions,
        reason = "The async ABI contract includes an opaque future return type."
    )]
    pub async extern "C" fn no_longer_unwind() {}
}

// Removing the u8 methods breaks assignments such as
// `let _: extern "C" fn() = RemovedOriginal::<u8>::now_unwind;`.
// The u16 methods keep their ABIs and must not trigger an ABI or unwind report.
pub struct RemovedOriginal<T>(pub T);
impl RemovedOriginal<u16> {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}

// With `ChangedAbi` in scope, calls on u8 resolve to the trait functions.
// For example, `TraitChanged::<u8>::now_unwind` now resolves to an
// `extern "C-unwind"` function. Assigning it to an `extern "C" fn()` fails.
// The u16 inherent functions keep their original ABIs, so none of the surviving
// inherent functions should trigger an ABI or unwind report.
pub struct TraitChanged<T, U = ()>(pub T, pub U);
impl TraitChanged<u16> {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}
impl ChangedAbi for TraitChanged<u8> {
    extern "Rust" fn changed_abi() {}
    extern "C-unwind" fn now_unwind() {}
    extern "C" fn no_longer_unwind() {}
}

// Once we can match the impls on u8, the ABI changes above should be reported
// even though this separate impl contains hidden and private methods.
impl Converged<u8> {
    fn private_unrelated() {}
    #[doc(hidden)]
    pub fn hidden_unrelated() {}
}

// The owner enters the supported API as its methods change ABI. Those methods
// had no supported baseline, so do not report ABI changes.
pub struct OwnerExposed;
impl OwnerExposed {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}

// Hiding the owner removes its methods from the supported API while their ABIs
// also change. Report the owner's availability loss without separate ABI reports.
#[doc(hidden)]
pub struct OwnerHidden;
impl OwnerHidden {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}

// Removing doc(hidden) exposes methods with changed ABIs. Their hidden baseline
// declarations cannot establish supported ABI contracts.
pub struct MethodExposed;
impl MethodExposed {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}

// Adding doc(hidden) removes methods from the supported API while their ABIs change.
// Report those availability losses without treating hidden methods as supported
// replacements for their old ABI contracts.
pub struct MethodHidden;
impl MethodHidden {
    #[doc(hidden)]
    pub extern "Rust" fn changed_abi() {}
    #[doc(hidden)]
    pub extern "C-unwind" fn now_unwind() {}
    #[doc(hidden)]
    pub extern "C" fn no_longer_unwind() {}
}

// Renaming these methods removes the old calls and introduces different ABIs.
// With no same-named candidate, report method removal rather than an ABI change.
pub struct RenamedMethod;
impl RenamedMethod {
    pub extern "Rust" fn renamed_changed_abi() {}
    pub extern "C-unwind" fn renamed_now_unwind() {}
    pub extern "C" fn renamed_no_longer_unwind() {}
}

// Gaining or losing unwind capability across ABI families should trigger only the ABI-name lint.
pub struct DifferentAbi;
impl DifferentAbi {
    pub extern "system" fn changed() {}
    pub extern "system-unwind" fn gains_unwind_with_abi_change() {}
}

// Replacing the struct with an enum changes the owner kind and its methods' ABIs.
// Diagnose the type replacement; the inherent ABI lints compare matching owner kinds.
pub enum OwnerKindChanged {}
impl OwnerKindChanged {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}

// Both u8 methods change from the C ABI family to the system ABI family.
// Neither changes its unwind behavior. The u16 methods keep their ABIs.
// TODO: Report the two ABI-name changes once we can match impl targets.
pub struct ChangesAbiFamily<T>(pub T);
impl ChangesAbiFamily<u8> {
    pub extern "system" fn read() {}
    pub extern "system-unwind" fn write() {}
}
impl ChangesAbiFamily<u16> {
    pub extern "C-unwind" fn read() {}
    pub extern "C" fn write() {}
}

// The new methods exist only on GainsParameter<1>. With `type Old = GainsParameter;`,
// `Old::exported()` now selects the default instantiation and loses the method.
pub struct GainsParameter<const N: usize = 0>;
impl GainsParameter<1> {
    #[unsafe(export_name = "disjoint_gains_parameter_new")]
    pub extern "system" fn exported() {}
    pub extern "C-unwind" fn gains_unwind() {}
    pub extern "C" fn loses_unwind() {}
}

// The old methods exist only on LosesParameter<1>. Removing the const parameter
// makes that type unavailable regardless of the new methods' ABIs or exports.
pub struct LosesParameter;
impl LosesParameter {
    #[unsafe(export_name = "disjoint_loses_parameter_new")]
    pub extern "system" fn exported() {}
    pub extern "C-unwind" fn gains_unwind() {}
    pub extern "C" fn loses_unwind() {}
}

// Each method exists on only one instantiation in each release.
// Replacing the methods on u8 with different methods on u16 breaks the u8
// calls, but no surviving method changes ABI, unwind behavior, or export name.
pub struct MovedMethods<T>(pub T);
impl MovedMethods<u16> {
    #[unsafe(export_name = "disjoint_moved_methods_new")]
    pub extern "system" fn exported() {}
    pub extern "C-unwind" fn gains_unwind() {}
    pub extern "C" fn loses_unwind() {}
}
