#![no_std]

// Renaming the u8 export removes the old symbol and breaks downstream linking.
// The u16 method keeps its own export name in both releases.
// TODO: Report the change on u8 once impl targets can be matched.
pub struct Renamed<T>(pub T);
impl Renamed<u8> {
    #[unsafe(export_name = "disjoint_renamed_new")]
    pub extern "C" fn exported() -> u32 {
        8
    }
}
impl Renamed<u16> {
    #[unsafe(export_name = "disjoint_renamed_unchanged")]
    pub extern "C" fn exported() -> u32 {
        16
    }
}

// Removing the u8 export attribute breaks linking against its old symbol.
// The Rust method remains callable and the u16 export keeps its name.
// TODO: Report the attribute removal on u8 once impl targets can be matched.
pub struct Removed<T>(pub T);
impl Removed<u8> {
    pub extern "C" fn exported() -> u32 {
        8
    }
}
impl Removed<u16> {
    #[unsafe(export_name = "disjoint_removed_unchanged")]
    pub extern "C" fn exported() -> u32 {
        16
    }
}

// Both methods change export names.
// TODO: Report both renames on their own declarations once impl targets can be matched.
pub struct BothChanged<T>(pub T);
impl BothChanged<u8> {
    #[unsafe(export_name = "disjoint_both_u8_new")]
    pub extern "C" fn exported() -> u32 {
        8
    }
}
impl BothChanged<u16> {
    #[unsafe(export_name = "disjoint_both_u16_new")]
    pub extern "C" fn exported() -> u32 {
        16
    }
}

// Each Rust method receives the other's symbol. The symbols still link but
// now return the other method's value, so the Rust-to-symbol mapping has changed.
// TODO: Report these method export changes once impl targets can be matched;
// the same-named methods still expose both old symbols.
pub struct Swapped<T>(pub T);
impl Swapped<u8> {
    #[unsafe(export_name = "disjoint_swap_u16")]
    pub extern "C" fn exported() -> u32 {
        8
    }
}
impl Swapped<u16> {
    #[unsafe(export_name = "disjoint_swap_u8")]
    pub extern "C" fn exported() -> u32 {
        16
    }
}

// The new u16 method adds a distinct symbol. The old u8 method and its symbol
// remain available, so no method export has changed.
pub struct Added<T>(pub T);
impl Added<u8> {
    #[unsafe(export_name = "disjoint_added_u8")]
    pub extern "C" fn exported() -> u32 {
        8
    }
}
impl Added<u16> {
    #[unsafe(export_name = "disjoint_added_u16")]
    pub extern "C" fn exported() -> u32 {
        16
    }
}

// The u8 symbol moves to a differently named method. The symbol still links,
// but the original method has lost its export name.
// TODO: Report that loss on u8 once impl targets can be matched.
pub struct MovedElsewhere<T>(pub T);
impl MovedElsewhere<u8> {
    pub extern "C" fn exported() -> u32 {
        8
    }
}
impl MovedElsewhere<u8> {
    #[unsafe(export_name = "disjoint_moved")]
    pub extern "C" fn other() -> u32 {
        24
    }
}
impl MovedElsewhere<u16> {
    #[unsafe(export_name = "disjoint_moved_unchanged")]
    pub extern "C" fn exported() -> u32 {
        16
    }
}

// The u8 methods leave the supported Rust API. One retains its symbol; the other renames it.
// Neither loss should be attributed to the unchanged public u16 methods.
// TODO: Attribute these changes once impl targets can be matched.
pub struct HiddenOriginal<T>(pub T);
impl HiddenOriginal<u8> {
    #[doc(hidden)]
    #[unsafe(export_name = "disjoint_hiddenoriginal_retained_old")]
    pub extern "C" fn retained() -> u32 {
        8
    }
}
impl HiddenOriginal<u16> {
    #[unsafe(export_name = "disjoint_hiddenoriginal_retained_unchanged")]
    pub extern "C" fn retained() -> u32 {
        16
    }
}
impl HiddenOriginal<u8> {
    #[doc(hidden)]
    #[unsafe(export_name = "disjoint_hiddenoriginal_renamed_new")]
    pub extern "C" fn renamed() -> u32 {
        8
    }

    // A public function with another name must not satisfy the public API check for `renamed`.
    pub fn unrelated() {}
}
impl HiddenOriginal<u16> {
    #[unsafe(export_name = "disjoint_hiddenoriginal_renamed_unchanged")]
    pub extern "C" fn renamed() -> u32 {
        16
    }
}

// The u8 methods leave the supported Rust API. One retains its symbol; the other renames it.
// Neither loss should be attributed to the unchanged public u16 methods.
// TODO: Attribute these changes once impl targets can be matched.
pub struct PrivateOriginal<T>(pub T);
impl PrivateOriginal<u8> {
    #[unsafe(export_name = "disjoint_privateoriginal_retained_old")]
    extern "C" fn retained() -> u32 {
        8
    }
}
impl PrivateOriginal<u16> {
    #[unsafe(export_name = "disjoint_privateoriginal_retained_unchanged")]
    pub extern "C" fn retained() -> u32 {
        16
    }
}
impl PrivateOriginal<u8> {
    #[unsafe(export_name = "disjoint_privateoriginal_renamed_new")]
    extern "C" fn renamed() -> u32 {
        8
    }
}
impl PrivateOriginal<u16> {
    #[unsafe(export_name = "disjoint_privateoriginal_renamed_unchanged")]
    pub extern "C" fn renamed() -> u32 {
        16
    }
}

// Only the hidden and private Rust methods change export names.
// Their symbols belong to the global FFI contract, but these methods
// are outside the supported Rust API and should not produce method export diagnostics.
pub struct ExcludedSources<T>(pub T);
impl ExcludedSources<u8> {
    #[unsafe(export_name = "disjoint_excluded_public")]
    pub extern "C" fn exported() -> u32 {
        8
    }
}
impl ExcludedSources<u16> {
    #[doc(hidden)]
    #[unsafe(export_name = "disjoint_excluded_hidden_new")]
    pub extern "C" fn exported() -> u32 {
        16
    }
}
impl ExcludedSources<u32> {
    #[unsafe(export_name = "disjoint_excluded_private_new")]
    extern "C" fn exported() -> u32 {
        32
    }
}

// Removing the u8 method removes its fixed symbol.
// The surviving u16 method keeps its export name.
pub struct RemovedOriginal<T>(pub T);
impl RemovedOriginal<u16> {
    #[unsafe(export_name = "disjoint_removed_method_unchanged")]
    pub extern "C" fn exported() -> u32 {
        16
    }
}

// `Exported` exists in both versions. The fallback cases assume
// it is already in scope at the call site.
pub trait Exported {
    extern "C" fn exported() -> u32;
}

// With `Exported` in scope, calls on u8 move to the existing trait impl.
// Its doc-hidden method remains callable and receives the old symbol.
// The Rust call and its export still return 8; the unrelated u16 method's different symbol
// must not be diagnosed as a replacement.
pub struct TraitPreserved<T>(pub T);
impl TraitPreserved<u16> {
    #[unsafe(export_name = "disjoint_trait_preserved_unchanged")]
    pub extern "C" fn exported() -> u32 {
        16
    }
}
impl Exported for TraitPreserved<u8> {
    #[unsafe(export_name = "disjoint_trait_preserved")]
    #[doc(hidden)]
    extern "C" fn exported() -> u32 {
        8
    }
}

// With `Exported` in scope, calls on u8 move to the existing trait impl.
// Linking against the old inherent symbol breaks, but no surviving inherent method
// changes its export name.
pub struct TraitChanged<T, U = ()>(pub T, pub U);
impl TraitChanged<u16> {
    #[unsafe(export_name = "disjoint_trait_changed_unchanged")]
    pub extern "C" fn exported() -> u32 {
        16
    }
}
impl Exported for TraitChanged<u8> {
    #[unsafe(export_name = "disjoint_trait_changed_new")]
    extern "C" fn exported() -> u32 {
        8
    }
}

// The inherent function still shadows the trait function. Moving its old symbol
// onto the trait impl preserves global symbol availability but changes the
// inherent method's mapping, which should be reported on this nongeneric owner.
pub struct TraitShadowed;
impl TraitShadowed {
    #[unsafe(export_name = "disjoint_trait_shadowed_new")]
    pub extern "C" fn exported() -> u32 {
        8
    }
}
impl Exported for TraitShadowed {
    #[unsafe(export_name = "disjoint_trait_shadowed_old")]
    extern "C" fn exported() -> u32 {
        16
    }
}

// Switching between no_mangle and an equal export_name preserves the symbol.
// The unrelated u16 exports must not turn either spelling change into a report.
pub struct SymbolSpelling<T>(pub T);
impl SymbolSpelling<u8> {
    #[unsafe(export_name = "disjoint_no_mangle_to_export")]
    pub extern "C" fn disjoint_no_mangle_to_export() -> u32 {
        8
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn disjoint_export_to_no_mangle() -> u32 {
        8
    }
}
impl SymbolSpelling<u16> {
    #[unsafe(export_name = "disjoint_spelling_u16_to_export")]
    pub extern "C" fn disjoint_no_mangle_to_export() -> u32 {
        16
    }
    #[unsafe(export_name = "disjoint_spelling_u16_to_no_mangle")]
    pub extern "C" fn disjoint_export_to_no_mangle() -> u32 {
        16
    }
}

// The u8 owner has a second impl whose hidden and private methods have
// different names from `exported`. Once we can match the impls on u8,
// `exported` should still be reported at the declaration whose export name changed.
impl Renamed<u8> {
    fn private_unrelated() {}
    #[doc(hidden)]
    pub fn hidden_unrelated() {}
}

// The owner enters the supported Rust API as its method receives a new export name.
// The old method was outside that API, so its changed export name is not reported.
pub struct OwnerExposed;
impl OwnerExposed {
    #[unsafe(export_name = "disjoint_ownerexposed_new")]
    pub extern "C" fn exported() -> u32 {
        8
    }
}

// The owner leaves the supported Rust API as its method receives a new export name.
// Report the owner's availability loss. The hidden owner cannot supply a supported
// replacement method, even though downstream linking also breaks.
#[doc(hidden)]
pub struct OwnerHidden;
impl OwnerHidden {
    #[unsafe(export_name = "disjoint_ownerhidden_new")]
    pub extern "C" fn exported() -> u32 {
        8
    }
}

// The method enters the supported Rust API with a new export name. Its hidden
// baseline declaration cannot establish a supported method-to-symbol mapping;
// the separate global FFI contract still loses the old symbol.
pub struct MethodExposed;
impl MethodExposed {
    #[unsafe(export_name = "disjoint_methodexposed_new")]
    pub extern "C" fn exported() -> u32 {
        8
    }
}

// The method leaves the supported Rust API and changes its export name.
// Report its availability loss. Its hidden declaration cannot establish a supported
// replacement mapping, even though downstream linking also breaks.
pub struct MethodHidden;
impl MethodHidden {
    #[doc(hidden)]
    #[unsafe(export_name = "disjoint_methodhidden_new")]
    pub extern "C" fn exported() -> u32 {
        8
    }
}

// Renaming the method and its symbol breaks the old Rust call and downstream linking.
// Report method removal; the export-name lint requires a same-named current function.
pub struct RenamedMethod;
impl RenamedMethod {
    #[unsafe(export_name = "disjoint_renamed_method_new")]
    pub extern "C" fn renamed() -> u32 {
        8
    }
}
