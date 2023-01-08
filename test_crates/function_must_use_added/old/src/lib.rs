// These functions did not have the #[must_use] attribute in the old version.
// Addition of the attribute should be reported by this rule.

pub fn FunctionToMustUseFunction(x: i64) -> i64 {
    x
}

pub fn FunctionToMustUseMessageFunction(x: i64) -> i64 {
    x
}


// These functions had the #[must_use] attribute in the old version. Changes of
// the attribute, including deletion, should not be reported by this rule.

#[must_use]
pub fn MustUseFunctionToFunction(x: i64) -> i64 {
    x
}

#[must_use]
pub fn MustUseFunctionToMustUseMessageFunction(x: i64) -> i64 {
    x
}


// These functions had the #[must_use] attribute in the old version.
// They also included the user-defined warning message. Changes of
// the attribute, including deletion, should not be reported by this rule.

#[must_use = "Foo"]
pub fn MustUseMessageFunctionToFunction(x: i64) -> i64 {
    x
}

#[must_use = "Foo"]
pub fn MustUseMessageFunctionToMustUseFunction(x: i64) -> i64 {
    x
}

#[must_use = "Foo"]
pub fn MustUseMessageFunctionToMustUseMessageFunction(x: i64) -> i64 {
    x
}


// This function is private and should not be reported by this rule.

fn MustUsePrivateFunction(x: i64) -> i64 {
    x
}
