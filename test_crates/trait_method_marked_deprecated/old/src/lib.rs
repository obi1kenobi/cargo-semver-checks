#![no_std]

pub trait TraitWithMethodToBeDeprecated {
    // This method will be marked deprecated
    fn method_to_deprecated(&self) -> i32;

    // This method will be marked deprecated with a message
    fn method_to_deprecated_message(&self, input: &str) -> &str;

    // These methods should not trigger the lint
    fn method_stays_normal(&self);

    #[deprecated]
    fn method_already_deprecated(&self);

    #[deprecated = "Old message"]
    fn method_message_changes(&self);
}

// Changes to private trait methods should not be reported
trait PrivateTrait {
    fn private_method_to_deprecated(&self);
}

#[deprecated]
pub trait DeprecatedTrait {
    // Adding deprecated to a method in an already deprecated trait should not be reported
    fn method_in_deprecated_trait(&self);
}

// Hidden trait - changes to its methods should not be reported
#[doc(hidden)]
pub trait HiddenTrait {
    fn method_becomes_deprecated(&self);
}

// Public trait with hidden method should not be reported
pub trait PublicTraitWithHiddenMethod {
    #[doc(hidden)]
    fn hidden_method_becomes_deprecated(&self);
}

// This trait will be marked deprecated - should not trigger the lint
pub trait TraitMarkedDeprecated {
    fn method(&self);
}

// Both the trait and its method will be marked deprecated - should not trigger the lint
pub trait TraitAndMethodWillBeDeprecated {
    fn method_will_be_deprecated(&self);
}
