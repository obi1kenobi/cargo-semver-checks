#![no_std]

mod root_hidden_glob_source {
    pub fn root_hidden_glob_only() {}
}

// The old crate's root `#[doc(hidden)]` glob is gone. Its direct path was not public API.

mod public_module_hidden_glob_source {
    pub fn public_module_hidden_glob_only() {}
}

// This public module remains, but its hidden glob re-export is gone too.
pub mod public_module_with_hidden_glob {}

mod root_hidden_per_item_source {
    pub fn root_hidden_per_item_only() {}
}

// The direct hidden per-item re-export is gone.

mod deprecated_hidden_glob_source {
    pub fn deprecated_hidden_glob_removed() {}
}

// Deprecated hidden re-exports were public API, so removing this glob should be reported.

mod deprecated_hidden_per_item_source {
    pub fn deprecated_hidden_per_item_removed() {}
}

// Removing this deprecated hidden per-item re-export should also be reported.

mod duplicate_hidden_and_public_glob_source {
    pub fn duplicate_hidden_and_public_glob_kept() {}
}

// The hidden duplicate glob is gone, but the public glob still preserves the public API.
pub use duplicate_hidden_and_public_glob_source::*;

mod duplicate_hidden_glob_public_per_item_source {
    pub fn duplicate_hidden_glob_public_per_item_kept() {}
}

// The public per-item re-export still preserves the public API.
pub use duplicate_hidden_glob_public_per_item_source::duplicate_hidden_glob_public_per_item_kept;

mod duplicate_hidden_per_item_public_glob_source {
    pub fn duplicate_hidden_per_item_public_glob_kept() {}
}

// The public glob still preserves the public API.
pub use duplicate_hidden_per_item_public_glob_source::*;

mod hidden_per_item_sources {
    pub fn hidden_per_item_then_public_per_item_removed() {}
    pub fn hidden_per_item_then_public_glob_removed() {}
}

mod hidden_per_item_for_public_per_item {}

mod hidden_per_item_for_public_glob {}

// The old crate's outward glob was not public API and now exports nothing.
pub use hidden_per_item_for_public_glob::*;

mod hidden_glob_for_public_per_item_source {
    pub fn hidden_glob_then_public_per_item_removed() {}
}

mod hidden_glob_for_public_per_item {}

mod hidden_glob_for_public_glob_source {
    pub fn hidden_glob_then_public_glob_removed() {}
}

mod hidden_glob_for_public_glob {}

// The old crate's outward glob was not public API and now exports nothing.
pub use hidden_glob_for_public_glob::*;

mod public_module_hidden_glob_public_glob_source {
    pub fn public_module_hidden_glob_then_public_glob_removed() {}
}

pub mod public_module_with_public_glob_from_hidden_glob {
    mod hidden_glob_layer {}

    // The outward public glob was not public API and now exports nothing.
    pub use hidden_glob_layer::*;
}

mod public_module_hidden_per_item_public_glob_source {
    pub fn public_module_hidden_per_item_then_public_glob_removed() {}
}

pub mod public_module_with_public_glob_from_hidden_per_item {
    mod hidden_per_item_layer {}

    // The outward public glob was not public API and now exports nothing.
    pub use hidden_per_item_layer::*;
}
