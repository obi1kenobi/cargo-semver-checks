#![no_std]

// These functions did not have the #[deprecated] attribute in the old version.
// Addition of the attribute should be reported by this rule.

#[deprecated]
pub fn function_to_deprecated() -> i32 {
    42
}

#[deprecated = "This attribute was added"]
pub fn function_to_deprecated_message(input: &str) -> &str {
    input
}

// These functions had the #[deprecated] attribute in the old version. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

pub fn deprecated_function_to_function(data: &[u8]) -> bool {
    true
}

#[deprecated = "This message was added"]
pub fn deprecated_function_to_deprecated_message() -> Option<i32> {
    None
}

// These functions had the #[deprecated] attribute in the old version.
// They also included the user-defined warning message. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

pub fn deprecated_message_function_to_function() {
    // Empty implementation
}

#[deprecated]
pub fn deprecated_message_function_to_deprecated() {
    // Empty implementation
}

#[deprecated = "This message was changed"]
pub fn deprecated_message_function_to_deprecated_message() {
    // Empty implementation
}

// This function is private and should NOT be reported by this rule.

#[deprecated]
fn deprecated_private_function() {
    // Empty implementation
}
