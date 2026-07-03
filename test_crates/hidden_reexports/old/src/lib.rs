#![no_std]

mod root_hidden_glob_source {
    pub fn root_hidden_glob_only() {}
}

// Items visible only through this root `#[doc(hidden)]` glob are not public API.
#[doc(hidden)]
pub use root_hidden_glob_source::*;

mod public_module_hidden_glob_source {
    pub fn public_module_hidden_glob_only() {}
}

// The same is true in public modules: downstream code could name this item directly,
// but the path relies on a `#[doc(hidden)]` glob re-export.
pub mod public_module_with_hidden_glob {
    #[doc(hidden)]
    pub use super::public_module_hidden_glob_source::*;
}

mod root_hidden_per_item_source {
    pub fn root_hidden_per_item_only() {}
}

// This test crate is specifically about `#[doc(hidden)]` glob behavior, but keep a
// per-item direct re-export as a control case showing the established behavior.
#[doc(hidden)]
pub use root_hidden_per_item_source::root_hidden_per_item_only;

mod deprecated_hidden_glob_source {
    pub fn deprecated_hidden_glob_removed() {}
}

// A deprecated hidden glob re-export still exposes public API.
#[deprecated]
#[doc(hidden)]
pub use deprecated_hidden_glob_source::*;

mod deprecated_hidden_per_item_source {
    pub fn deprecated_hidden_per_item_removed() {}
}

// The same deprecated-hidden exception applies to per-item re-exports.
#[deprecated]
#[doc(hidden)]
pub use deprecated_hidden_per_item_source::deprecated_hidden_per_item_removed;

mod duplicate_hidden_and_public_glob_source {
    pub fn duplicate_hidden_and_public_glob_kept() {}
}

// If the same item is visible through both a hidden glob and a public glob, the public
// glob path is public API. Removing only the hidden glob should not be reported.
#[doc(hidden)]
pub use duplicate_hidden_and_public_glob_source::*;
pub use duplicate_hidden_and_public_glob_source::*;

mod duplicate_hidden_glob_public_per_item_source {
    pub fn duplicate_hidden_glob_public_per_item_kept() {}
}

// A public per-item re-export also keeps the item public API when the hidden glob is deleted.
#[doc(hidden)]
pub use duplicate_hidden_glob_public_per_item_source::*;
pub use duplicate_hidden_glob_public_per_item_source::duplicate_hidden_glob_public_per_item_kept;

mod duplicate_hidden_per_item_public_glob_source {
    pub fn duplicate_hidden_per_item_public_glob_kept() {}
}

// A public glob re-export also keeps the item public API when a hidden per-item re-export
// of the same item is deleted.
#[doc(hidden)]
pub use duplicate_hidden_per_item_public_glob_source::duplicate_hidden_per_item_public_glob_kept;
pub use duplicate_hidden_per_item_public_glob_source::*;

mod hidden_per_item_sources {
    pub fn hidden_per_item_then_public_per_item_removed() {}
    pub fn hidden_per_item_then_public_glob_removed() {}
}

mod hidden_per_item_for_public_per_item {
    #[doc(hidden)]
    pub use super::hidden_per_item_sources::hidden_per_item_then_public_per_item_removed;
}

// This name is public API because downstream code imports it through this public re-export.
// The hidden per-item re-export above is just an internal implementation detail.
pub use hidden_per_item_for_public_per_item::hidden_per_item_then_public_per_item_removed;

mod hidden_per_item_for_public_glob {
    #[doc(hidden)]
    pub use super::hidden_per_item_sources::hidden_per_item_then_public_glob_removed;
}

// This outward glob relies on the hidden per-item re-export, so the name is not public API.
pub use hidden_per_item_for_public_glob::*;

mod hidden_glob_for_public_per_item_source {
    pub fn hidden_glob_then_public_per_item_removed() {}
}

mod hidden_glob_for_public_per_item {
    #[doc(hidden)]
    pub use super::hidden_glob_for_public_per_item_source::*;
}

// This explicit public per-item re-export creates a public API path. The hidden glob
// inside the module only resolves the re-export target; downstream users don't type it.
pub use hidden_glob_for_public_per_item::hidden_glob_then_public_per_item_removed;

mod hidden_glob_for_public_glob_source {
    pub fn hidden_glob_then_public_glob_removed() {}
}

mod hidden_glob_for_public_glob {
    #[doc(hidden)]
    pub use super::hidden_glob_for_public_glob_source::*;
}

// This outward glob relies on a hidden glob, so the synthesized path is not public API.
pub use hidden_glob_for_public_glob::*;

mod public_module_hidden_glob_public_glob_source {
    pub fn public_module_hidden_glob_then_public_glob_removed() {}
}

pub mod public_module_with_public_glob_from_hidden_glob {
    mod hidden_glob_layer {
        #[doc(hidden)]
        pub use super::super::public_module_hidden_glob_public_glob_source::*;
    }

    // The outward glob is public, but the hidden glob inside `hidden_glob_layer`
    // is what makes the name visible to it, so this is not public API.
    pub use hidden_glob_layer::*;
}

mod public_module_hidden_per_item_public_glob_source {
    pub fn public_module_hidden_per_item_then_public_glob_removed() {}
}

pub mod public_module_with_public_glob_from_hidden_per_item {
    mod hidden_per_item_layer {
        #[doc(hidden)]
        pub use super::super::public_module_hidden_per_item_public_glob_source::{
            public_module_hidden_per_item_then_public_glob_removed,
        };
    }

    // A public-module outward glob over an internal hidden per-item re-export is not public API.
    pub use hidden_per_item_layer::*;
}
