pub trait TraitWithTypeToBeDeprecated {
    // This associated type will be marked deprecated
    type TypeToDeprecated: Clone;

    // This associated type will be marked deprecated with a message
    type TypeToDeprecatedMessage: ToString;

    // These types should not trigger the lint
    type TypeStaysNormal;

    #[deprecated]
    type TypeAlreadyDeprecated;

    #[deprecated = "old message"]
    type TypeMessageChanges;
}

// Changes to private trait types should not be reported
trait PrivateTrait {
    type PrivateTypeToDeprecated;
}

#[deprecated]
pub trait DeprecatedTrait {
    // Adding deprecated to a type in an already deprecated trait should not be reported
    type TypeInDeprecatedTrait;
}

// Hidden trait - changes to its types should not be reported
#[doc(hidden)]
pub trait HiddenTrait {
    type TypeBecomesDeprecated;
}

// Public trait with hidden type should not be reported
pub trait PublicTraitWithHiddenType {
    #[doc(hidden)]
    type HiddenTypeBecomesDeprecated;
}

// This trait will be marked deprecated - should not trigger the lint
pub trait TraitMarkedDeprecated {
    type Type;
}

// Both the trait and its type will be marked deprecated - should not trigger the lint
pub trait TraitAndTypeWillBeDeprecated {
    type TypeWillBeDeprecated;
}
