pub trait TraitWithMethodToBeDeprecated {
    // These methods are now deprecated and should be reported
    #[deprecated]
    fn method_to_deprecated(&self) -> i32;

    #[deprecated = "This method is deprecated"]
    fn method_to_deprecated_message(&self, input: &str) -> String;

    // These methods should not trigger the lint
    fn method_stays_normal(&self);

    #[deprecated]
    fn method_already_deprecated(&self);

    #[deprecated = "New message"]
    fn method_message_changes(&self);
}

// Changes to private trait methods should not be reported
trait PrivateTrait {
    #[deprecated]
    fn private_method_to_deprecated(&self);
}

#[deprecated]
pub trait DeprecatedTrait {
    // Adding deprecated to a method in an already deprecated trait should not be reported
    #[deprecated]
    fn method_in_deprecated_trait(&self);
}

// This trait was added in the new version with deprecated methods.
// It should NOT be reported by this rule to avoid duplicate lints.
pub trait NewTraitWithDeprecatedMethod {
    #[deprecated]
    fn new_deprecated_method(&self);
}
