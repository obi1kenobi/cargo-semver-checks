pub trait TraitWithConstToBeDeprecated {
    // This associated constant will be marked deprecated
    const CONST_TO_DEPRECATED: i32 = 42;

    // This associated constant will be marked deprecated with a message
    const CONST_TO_DEPRECATED_MESSAGE: &'static str = "hello";

    // These constants should not trigger the lint
    const CONST_STAYS_NORMAL: bool = true;

    #[deprecated]
    const CONST_ALREADY_DEPRECATED: u32 = 100;

    #[deprecated = "Old message"]
    const CONST_MESSAGE_CHANGES: char = 'x';
}

// Changes to private trait constants should not be reported
trait PrivateTrait {
    const PRIVATE_CONST_TO_DEPRECATED: i32 = 0;
}

#[deprecated]
pub trait DeprecatedTrait {
    // Adding deprecated to a constant in an already deprecated trait should not be reported
    const CONST_IN_DEPRECATED_TRAIT: i32 = 1;
}

// Hidden trait - changes to its constants should not be reported
#[doc(hidden)]
pub trait HiddenTrait {
    const CONST_BECOMES_DEPRECATED: i32 = 2;
}

// Public trait with hidden constant should not be reported
pub trait PublicTraitWithHiddenConst {
    #[doc(hidden)]
    const HIDDEN_CONST_BECOMES_DEPRECATED: i32 = 3;
}
