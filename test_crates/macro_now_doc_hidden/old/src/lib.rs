#![no_std]

/// A macro that will become hidden
#[macro_export]
macro_rules! will_be_hidden {
    ($val:expr, $opts:expr) => {
        println!("Processing {} with {:?}", $val, $opts);
    };
}

/// A macro that stays public
#[macro_export]
macro_rules! stays_public {
    () => {
        println!("This macro remains public");
    };
}

// Already hidden macro that changes implementation but stays hidden
#[doc(hidden)]
#[macro_export]
macro_rules! already_hidden {
    () => {
        println!("Version 1");
    };
}

// Non-exported macro that becomes hidden - should not trigger
/// Some documentation
macro_rules! non_exported_becomes_hidden {
    () => {
        println!("Not exported");
    };
}

// Exported macro that becomes non-exported but not doc(hidden) - should not trigger
#[macro_export]
macro_rules! becomes_non_exported {
    () => {
        println!("Will become non-exported");
    };
}
