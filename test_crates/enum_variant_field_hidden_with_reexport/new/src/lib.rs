#![no_std]

mod implementation {
    pub enum SolePath {
        Tuple(#[doc(hidden)] u8),
        Struct {
            #[doc(hidden)]
            field: u8,
        },
    }

    pub enum PreservedPath {
        Tuple(#[doc(hidden)] u8),
        Struct {
            #[doc(hidden)]
            field: u8,
        },
    }

    pub enum ReplacedPaths {
        Tuple(#[doc(hidden)] u8),
        Struct {
            #[doc(hidden)]
            field: u8,
        },
    }
}

// The sole public path became hidden: only `enum_now_doc_hidden` should report.
#[doc(hidden)]
pub use implementation::SolePath;

// Report the hidden fields only through RetainedAlias, which was public in both versions.
// `enum_now_doc_hidden` should also report PreservedPath becoming hidden.
#[doc(hidden)]
pub use implementation::PreservedPath;
pub use implementation::PreservedPath as RetainedAlias;

// Both old paths became hidden, so only `enum_now_doc_hidden` should report them.
// NewAlias could not be used in the baseline and must not trigger field-hidden diagnostics.
#[doc(hidden)]
pub use implementation::ReplacedPaths;
#[doc(hidden)]
pub use implementation::ReplacedPaths as OldAlias;
pub use implementation::ReplacedPaths as NewAlias;
