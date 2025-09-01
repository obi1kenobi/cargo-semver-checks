#![no_std]

// These functions had the #[must_use] attribute in the old version.
// Removal of the attribute should be reported by this rule.

#[must_use]
pub fn MustUseFunctionToFunction() {}

#[must_use = "Foo"]
pub fn MustUseMessageFunctionToFunction() {}


// These functions did not have the #[must_use] attribute in the old version.
// Addition of the attribute should NOT be reported by this rule.

pub fn FunctionToMustUseFunction() {}

#[must_use = "Foo"]
pub fn FunctionToMustUseMessageFunction() {}


// These functions change from one form of #[must_use] to another.
// They should NOT be reported by this rule.

#[must_use]
pub fn MustUseFunctionToMustUseMessageFunction() {}

#[must_use = "Foo"]
pub fn MustUseMessageFunctionToMustUseFunction() {}

#[must_use = "Foo"]
pub fn MustUseMessageFunctionToMustUseMessageFunction() {}


// This function is private and should NOT be reported by this rule.

#[must_use]
fn MustUsePrivateFunction() {}


// This function was removed in the new version of the crate with its attribute.
// It should NOT be reported by this rule to avoid duplicate lints.
// It should be reported as a removed pub item that is part of the crate's API.

#[must_use]
pub fn MustUseRemovedFunction() {}
