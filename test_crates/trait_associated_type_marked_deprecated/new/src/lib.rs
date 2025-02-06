pub trait TraitWithTypeToBeDeprecated {
    // These types are now deprecated and should be reported
    #[deprecated]
    type TypeToDeprecated: Clone;

    #[deprecated = "This type is deprecated"]
    type TypeToDeprecatedMessage: ToString;

    // These types should not trigger the lint
    type TypeStaysNormal;

    #[deprecated]
    type TypeAlreadyDeprecated;

    #[deprecated = "New message"]
    type TypeMessageChanges;
}

// Changes to private trait types should not be reported
trait PrivateTrait {
    #[deprecated]
    type PrivateTypeToDeprecated;
}

#[deprecated]
pub trait DeprecatedTrait {
    // Adding deprecated to a type in an already deprecated trait should not be reported
    #[deprecated]
    type TypeInDeprecatedTrait;
}

// Hidden trait - changes to its types should not be reported
#[doc(hidden)]
pub trait HiddenTrait {
    #[deprecated]
    type TypeBecomesDeprecated;
}

// Public trait with hidden type should not be reported
pub trait PublicTraitWithHiddenType {
    #[doc(hidden)]
    #[deprecated]
    type HiddenTypeBecomesDeprecated;
}

// This trait will be marked deprecated - should not trigger the lint
#[deprecated]
pub trait TraitMarkedDeprecated {
    type Type;
}

// Both the trait and its type will be marked deprecated - should not trigger the lint
#[deprecated]
pub trait TraitAndTypeWillBeDeprecated {
    #[deprecated]
    type TypeWillBeDeprecated;
}
