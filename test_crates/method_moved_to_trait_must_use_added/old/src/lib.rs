// The purpose of this test crate is to avoid duplicate lints. In this crate,
// there could be reports about both #[must_use] being added to the methods,
// as well as the methods themselves being moved to a Trait. 
// Because both of these are minor changes, and the #[must_use] added violation
// is impossible to achieve here without the method_moved_to_trait check failing,
// we want the #[must_use] checks to not find any changes, but expect a failure
// on the method_moved_to_trait check.

// This crate's test cases were separated into smaller files for easier
// management and debugging.

pub mod enum_method_moved_to_trait_must_use_added;

pub mod struct_method_moved_to_trait_must_use_added;

pub mod union_method_moved_to_trait_must_use_added;
