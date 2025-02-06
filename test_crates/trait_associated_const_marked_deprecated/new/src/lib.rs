pub trait TraitWithConstToBeDeprecated {
    // These constants are now deprecated and should be reported
    #[deprecated]
    const CONST_TO_DEPRECATED: i32 = 42;

    #[deprecated = "This constant is deprecated"]
    const CONST_TO_DEPRECATED_MESSAGE: &'static str = "hello";

    // These constants should not trigger the lint
    const CONST_STAYS_NORMAL: bool = true;

    #[deprecated]
    const CONST_ALREADY_DEPRECATED: u32 = 100;

    #[deprecated = "New message"]
    const CONST_MESSAGE_CHANGES: char = 'x';
}

// Changes to private trait constants should not be reported
trait PrivateTrait {
    #[deprecated]
    const PRIVATE_CONST_TO_DEPRECATED: i32 = 0;
}

#[deprecated]
pub trait DeprecatedTrait {
    // Adding deprecated to a constant in an already deprecated trait should not be reported
    #[deprecated]
    const CONST_IN_DEPRECATED_TRAIT: i32 = 1;
}

// Hidden trait - changes to its constants should not be reported
#[doc(hidden)]
pub trait HiddenTrait {
    #[deprecated]
    const CONST_BECOMES_DEPRECATED: i32 = 2;
}

// Public trait with hidden constant should not be reported
pub trait PublicTraitWithHiddenConst {
    #[doc(hidden)]
    #[deprecated]
    const HIDDEN_CONST_BECOMES_DEPRECATED: i32 = 3;
}

// This trait will be marked deprecated - should not trigger the lint
#[deprecated]
pub trait TraitMarkedDeprecated {
    const CONST: i32;
}

// Both the trait and its constant will be marked deprecated - should not trigger the lint
#[deprecated]
pub trait TraitAndConstWillBeDeprecated {
    #[deprecated]
    const CONST_WILL_BE_DEPRECATED: i32;
}
