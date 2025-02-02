// These variants did not have the #[deprecated] attribute in the old version.
// Addition of the attribute should be reported by this rule.
pub enum Status {
    Valid,
    #[deprecated]
    Disabled,  // Basic deprecation
    
    #[deprecated = "Use Inactive instead"]
    Deleted,   // Deprecation with message
    
    #[deprecated(since = "1.1.0", note = "Use Inactive - this will be removed in 2.0")]
    Expired,   // Deprecation with metadata
    
    Inactive
}

// These variants had deprecation in the old version.
// Changes or removal should NOT be reported by this rule.
pub enum PreviouslyDeprecated {
    Current,
    Old,               // Was deprecated, now not
    #[deprecated = "New message"]
    Changed,           // Changed message
    #[deprecated]
    StillDeprecated,   // Changed form but still deprecated
}

// Private enum should not trigger the lint
enum PrivateEnum {
    #[deprecated]
    OldVariant,
    NewVariant
}