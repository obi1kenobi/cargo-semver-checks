#![no_std]

// These functions did not have the #[must_use] attribute in the old version.
// Addition of the attribute should be reported by this rule.

pub fn FunctionToMustUseFunction() {}

pub fn FunctionToMustUseMessageFunction() {}


// These functions had the #[must_use] attribute in the old version. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

#[must_use]
pub fn MustUseFunctionToFunction() {}

#[must_use]
pub fn MustUseFunctionToMustUseMessageFunction() {}


// These functions had the #[must_use] attribute in the old version.
// They also included the user-defined warning message. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

#[must_use = "Foo"]
pub fn MustUseMessageFunctionToFunction() {}

#[must_use = "Foo"]
pub fn MustUseMessageFunctionToMustUseFunction() {}

#[must_use = "Foo"]
pub fn MustUseMessageFunctionToMustUseMessageFunction() {}


// This function is private and should NOT be reported by this rule.

fn MustUsePrivateFunction() {}
