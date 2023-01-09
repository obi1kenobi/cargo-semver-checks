// These functions did not have the #[must_use] attribute in the old version.
// Addition of the attribute should be reported by this rule.

#[must_use]
pub fn FunctionToMustUseFunction() {}

#[must_use = "Foo"]
pub fn FunctionToMustUseMessageFunction() {}


// These functions had the #[must_use] attribute in the old version. Changes of
// the attribute, including deletion, should not be reported by this rule.

pub fn MustUseFunctionToFunction() {}

#[must_use = "Foo"]
pub fn MustUseFunctionToMustUseMessageFunction() {}


// These functions had the #[must_use] attribute in the old version.
// They also included the user-defined warning message. Changes of
// the attribute, including deletion, should not be reported by this rule.

pub fn MustUseMessageFunctionToFunction() {}

#[must_use]
pub fn MustUseMessageFunctionToMustUseFunction() {}

#[must_use = "Baz"]
pub fn MustUseMessageFunctionToMustUseMessageFunction() {}


// This function is private and should not be reported by this rule.

#[must_use]
fn MustUsePrivateFunction() {}