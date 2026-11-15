#![no_std]

// Only the u8 methods change ABI. Assigning them to function pointers with
// their old ABIs breaks, although the u16 methods retain those ABIs.
// TODO: Report the u8 changes once impl targets can be matched. Until then,
// the retained ABI contracts make these changes ambiguous.
pub struct Partial<T>(pub T);
impl Partial<u8> {
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
    pub extern "C-unwind" fn no_longer_unwind() {}
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
    pub extern "C" fn changed_abi() {}
    #[unsafe(export_name = "disjoint_abi_swapped_u8_now_unwind")]
    pub extern "C" fn now_unwind() {}
    #[unsafe(export_name = "disjoint_abi_swapped_u8_no_longer_unwind")]
    pub extern "C-unwind" fn no_longer_unwind() {}
}
impl Swapped<u16> {
    #[unsafe(export_name = "disjoint_abi_swapped_u16_changed_abi")]
    pub extern "Rust" fn changed_abi() {}
    #[unsafe(export_name = "disjoint_abi_swapped_u16_now_unwind")]
    pub extern "C-unwind" fn now_unwind() {}
    #[unsafe(export_name = "disjoint_abi_swapped_u16_no_longer_unwind")]
    pub extern "C" fn no_longer_unwind() {}
}

// Only the u8 methods change, and their old ABI contracts disappear entirely.
// Report each old u8 declaration once. The unchanged u16 methods must neither
// duplicate diagnostics nor be identified as the declarations that changed.
pub struct Converged<T>(pub T);
impl Converged<u8> {
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
    pub extern "C-unwind" fn no_longer_unwind() {}
}
impl Converged<u16> {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}

// Both methods with each name change ABI. Report each old declaration once,
// rather than every possible pairing of old and current declarations.
pub struct BothChanged<T>(pub T);
impl BothChanged<u8> {
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
    pub extern "C-unwind" fn no_longer_unwind() {}
}
impl BothChanged<u16> {
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
    pub extern "C-unwind" fn no_longer_unwind() {}
}

// The u8 methods lose their old ABI contracts. The unchanged u16 methods use system ABIs,
// so matching unwind flags must not count as preserving the u8 methods' old C ABI contracts.
// Report the u8 ABI losses without attributing them to the unchanged u16 methods.
pub struct ThirdAbi<T>(pub T);
impl ThirdAbi<u8> {
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
    pub extern "C-unwind" fn no_longer_unwind() {}
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

// The u8 methods leave the supported API without changing their ABIs.
// Do not mistake the unchanged u16 methods for their replacements.
// TODO: Attribute the API loss to u8 once impl targets can be matched.
pub struct HiddenOriginal<T>(pub T);
impl HiddenOriginal<u8> {
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
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
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
    pub extern "C-unwind" fn no_longer_unwind() {}
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
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
    pub extern "C-unwind" fn no_longer_unwind() {}
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
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
    pub extern "C-unwind" fn no_longer_unwind() {}
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
    pub extern "C" fn changed_abi() {}
    #[doc(hidden)]
    pub extern "C" fn now_unwind() {}
    #[doc(hidden)]
    pub extern "C-unwind" fn no_longer_unwind() {}
}
impl ExcludedSources<u32> {
    extern "C" fn changed_abi() {}
    extern "C" fn now_unwind() {}
    extern "C-unwind" fn no_longer_unwind() {}
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
impl TraitPreserved<u8> {
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
    pub extern "C-unwind" fn no_longer_unwind() {}
}
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
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
    pub extern "C-unwind" fn no_longer_unwind() {}
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
impl TraitFallback {
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
    pub extern "C-unwind" fn no_longer_unwind() {}
}
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
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
    pub extern "C-unwind" fn no_longer_unwind() {}

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
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
    pub extern "C-unwind" fn no_longer_unwind() {}
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
    #[expect(
        improper_ctypes_definitions,
        reason = "The async ABI contract includes an opaque future return type."
    )]
    pub async extern "C" fn changed_abi() {}
    #[expect(
        improper_ctypes_definitions,
        reason = "The async ABI contract includes an opaque future return type."
    )]
    pub async extern "C" fn now_unwind() {}
    #[expect(
        improper_ctypes_definitions,
        reason = "The async ABI contract includes an opaque future return type."
    )]
    pub async extern "C-unwind" fn no_longer_unwind() {}
}

// Removing the u8 methods loses their ABI contracts altogether. The remaining
// u16 methods keep their existing ABIs, so report the lost baseline capabilities
// without describing those surviving declarations as having changed.
pub struct RemovedOriginal<T>(pub T);
impl RemovedOriginal<u8> {
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
    pub extern "C-unwind" fn no_longer_unwind() {}
}
impl RemovedOriginal<u16> {
    pub extern "Rust" fn changed_abi() {}
    pub extern "C-unwind" fn now_unwind() {}
    pub extern "C" fn no_longer_unwind() {}
}

// With `ChangedAbi` in scope, calls on u8 fall back to that trait
// when the inherent functions disappear.
// That trait's different ABI contracts cannot preserve the old pointer assignments.
// Report the lost baseline capabilities, without identifying the unchanged
// u16 inherent functions or trait functions as their replacements.
pub struct TraitChanged<T, U = ()>(pub T, pub U);
impl TraitChanged<u8> {
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
    pub extern "C-unwind" fn no_longer_unwind() {}
}
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

// These unrelated methods leave the public u8 ABI losses on Converged detectable.
// An impl with no relevant method cannot make that method's availability ambiguous.
impl Converged<u8> {
    fn private_unrelated() {}
    #[doc(hidden)]
    pub fn hidden_unrelated() {}
}

// The owner enters the supported API as its methods change ABI. Those methods
// had no supported baseline, so do not report ABI changes.
#[doc(hidden)]
pub struct OwnerExposed;
impl OwnerExposed {
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
    pub extern "C-unwind" fn no_longer_unwind() {}
}

// Hiding the owner removes its methods from the supported API while their ABIs
// also change. Report the owner's availability loss without separate ABI reports.
pub struct OwnerHidden;
impl OwnerHidden {
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
    pub extern "C-unwind" fn no_longer_unwind() {}
}

// Removing doc(hidden) exposes methods with changed ABIs. Their hidden baseline
// declarations cannot establish supported ABI contracts.
pub struct MethodExposed;
impl MethodExposed {
    #[doc(hidden)]
    pub extern "C" fn changed_abi() {}
    #[doc(hidden)]
    pub extern "C" fn now_unwind() {}
    #[doc(hidden)]
    pub extern "C-unwind" fn no_longer_unwind() {}
}

// Adding doc(hidden) removes methods from the supported API while their ABIs change.
// Report those availability losses without treating hidden methods as supported
// replacements for their old ABI contracts.
pub struct MethodHidden;
impl MethodHidden {
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
    pub extern "C-unwind" fn no_longer_unwind() {}
}

// Renaming these methods removes the old calls and introduces different ABIs.
// With no same-named candidate, report method removal rather than an ABI change.
pub struct RenamedMethod;
impl RenamedMethod {
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
    pub extern "C-unwind" fn no_longer_unwind() {}
}

// This method changes its normalized ABI name as well as its unwind contract.
// Report only the ABI-name change; the unwind lints require the same ABI family.
pub struct DifferentAbi;
impl DifferentAbi {
    pub extern "C-unwind" fn changed() {}
}

// Replacing the struct with an enum changes the owner kind and its methods' ABIs.
// Diagnose the type replacement; the inherent ABI lints compare matching owner kinds.
pub struct OwnerKindChanged;
impl OwnerKindChanged {
    pub extern "C" fn changed_abi() {}
    pub extern "C" fn now_unwind() {}
    pub extern "C-unwind" fn no_longer_unwind() {}
}
