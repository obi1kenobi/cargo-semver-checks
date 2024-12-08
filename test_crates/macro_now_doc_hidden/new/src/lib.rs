/// Now hidden from docs
#[doc(hidden)]
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
        println!("Version 2");
    };
}

// Non-exported macro that becomes hidden - should not trigger
#[doc(hidden)]
macro_rules! non_exported_becomes_hidden {
    () => {
        println!("Not exported and now hidden");
    };
}

// Macro that was exported but is no longer exported - should not trigger
macro_rules! becomes_non_exported {
    () => {
        println!("No longer exported");
    };
}
