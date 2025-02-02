// These variants did not have the #[deprecated] attribute in the old version.
// Addition of the attribute should be reported by this rule.
pub enum Status {
    Valid,
    Disabled,    // Will get basic deprecation
    Deleted,     // Will get message deprecation
    Expired,     // Will get full metadata deprecation
    Inactive
}

// These variants had deprecation in the old version.
// Changes or removal should NOT be reported by this rule.
pub enum PreviouslyDeprecated {
    Current,
    #[deprecated]
    Old,                               // Will lose deprecation
    #[deprecated = "Original message"] 
    Changed,                           // Will change message
    #[deprecated = "Will change form"]
    StillDeprecated,                   // Will change form but stay deprecated
}

// Private enum should not trigger the lint
enum PrivateEnum {
    OldVariant,
    NewVariant
}