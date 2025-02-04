// These macros will be marked deprecated
#[macro_export]
macro_rules! macro_to_deprecated {
    () => {
        42
    };
}

#[macro_export]
macro_rules! macro_to_deprecated_message {
    ($x:expr) => {
        $x + 1
    };
}

// These macros should not trigger the lint
#[macro_export]
macro_rules! macro_stays_normal {
    ($x:expr) => {
        $x.to_string()
    };
}

#[deprecated]
#[macro_export]
macro_rules! macro_already_deprecated {
    () => {
        true
    };
}

#[deprecated = "Old message"]
#[macro_export]
macro_rules! macro_message_changes {
    () => {
        "hello"
    };
}

// Private macros should not be reported
macro_rules! private_macro_to_deprecated {
    () => {
        0
    };
}

// Hidden macros should not be reported
#[doc(hidden)]
#[macro_export]
macro_rules! hidden_macro_to_deprecated {
    () => {
        1
    };
}

mod foo {
    // Public macro. Exported even though it's in a private module,
    // because of the `#[macro_export]`.
    #[macro_export]
    macro_rules! inner_macro_exported_to_deprecated {
        () => {
            100
        };
    }
}

mod bar {
    macro_rules! inner_macro_to_deprecated {
        () => {
            100
        };
    }
}
