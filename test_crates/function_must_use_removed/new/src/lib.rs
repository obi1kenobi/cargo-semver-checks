#![no_std]

// These functions had the #[must_use] attribute in the old version.
// Removal of the attribute should be reported by this rule.

pub fn MustUseFunctionToFunction() {}

pub fn MustUseMessageFunctionToFunction() {}


// These functions did not have the #[must_use] attribute in the old version.
// Addition of the attribute should NOT be reported by this rule.

#[must_use]
pub fn FunctionToMustUseFunction() {}

#[must_use = "Foo"]
pub fn FunctionToMustUseMessageFunction() {}


// These functions change from one form of #[must_use] to another.
// They should NOT be reported by this rule.

#[must_use = "Foo"]
pub fn MustUseFunctionToMustUseMessageFunction() {}

#[must_use]
pub fn MustUseMessageFunctionToMustUseFunction() {}

#[must_use = "Baz"]
pub fn MustUseMessageFunctionToMustUseMessageFunction() {}


// This function is private and should NOT be reported by this rule.

fn MustUsePrivateFunction() {}
