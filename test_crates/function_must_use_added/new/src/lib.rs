// These functions did not have the #[must_use] attribute in the old version.
// Addition of the attribute should be reported by this rule.

#[must_use]
pub fn FunctionToMustUseFunction() {}

#[must_use = "Foo"]
pub fn FunctionToMustUseMessageFunction() {}


// These functions had the #[must_use] attribute in the old version. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

pub fn MustUseFunctionToFunction() {}

#[must_use = "Foo"]
pub fn MustUseFunctionToMustUseMessageFunction() {}


// These functions had the #[must_use] attribute in the old version.
// They also included the user-defined warning message. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

pub fn MustUseMessageFunctionToFunction() {}

#[must_use]
pub fn MustUseMessageFunctionToMustUseFunction() {}

#[must_use = "Baz"]
pub fn MustUseMessageFunctionToMustUseMessageFunction() {}


// This function is private and should NOT be reported by this rule.

#[must_use]
fn MustUsePrivateFunction() {}


// This function was added in the new version of the crate with its attribute.
// It should NOT be reported by this rule to avoid duplicate lints.
// It should be reported as a new pub item that is part of the crate's API.

#[must_use]
pub fn MustUseNewFunction() {}
